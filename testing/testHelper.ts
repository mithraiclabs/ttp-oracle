import {
  Account,
  BpfLoader,
  BPF_LOADER_PROGRAM_ID,
  Connection,
  PublicKey,
} from '@solana/web3.js';
import fs from 'mz/fs';
/**
 * This script should be run before the jest suite starts running tests on the contracts.
 *  1) establish a connection to localnet
 *  2) create an array of accounts with a balance (add to global)
 *  3) Deploy contracts
 *
 */
const DIST_DIRECTORY = './dist';
export const PROGRAM_PATHS: Record<string, string> = fs
  .readdirSync(DIST_DIRECTORY)
  .reduce((acc, fileName) => {
    const path = `${DIST_DIRECTORY}/${fileName}`;
    acc[fileName] = path;
    return acc;
  }, {} as Record<string, string>);
export const LOCALNET_URL = 'http://localhost:8899';

export const sleep = (ms: number): Promise<void> => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

export type NewAccountOptions = {
  lamports: number;
  noAirdropCheck: boolean;
};
/**
 * Create a new account with an airdropped amount of tokens
 * @param connection
 * @param {NewAccountOptions} options
 */
export const newAccountWithLamports = async (
  connection: Connection,
  options: NewAccountOptions = {
    lamports: 1000000,
    noAirdropCheck: false,
  },
): Promise<Account> => {
  const account = new Account();

  if (options.noAirdropCheck) {
    connection.requestAirdrop(account.publicKey, options.lamports);
    return account;
  }

  let retries = 10;
  await connection.requestAirdrop(account.publicKey, options.lamports);
  for (;;) {
    await sleep(500);
    if (options.lamports == (await connection.getBalance(account.publicKey))) {
      return account;
    }
    if (--retries <= 0) {
      break;
    }
  }
  throw new Error(`Airdrop of ${options.lamports} failed`);
};

export default class TestHelper {
  connection!: Connection;
  accounts: Array<Account> = [];
  programs: Record<string, PublicKey> = {};

  constructor(programs?: Record<string, PublicKey>) {
    if (programs) {
      this.programs = programs;
    }
    this.establishConnection();
  }
  /**
   * establishes connection to Solana cluster
   * @param url
   */
  establishConnection(url: string = LOCALNET_URL): void {
    this.connection = new Connection(url, 'recent');
  }
  /**
   * Creates 10 accounts for help with tests
   */
  async createAccounts(): Promise<void> {
    let numberOfAccounts = 10;
    for (;;) {
      const account = await newAccountWithLamports(this.connection, {
        // create accounts with a lot of lamports for testing purposes
        lamports: 10000000000000,
        noAirdropCheck: true,
      });
      this.accounts.push(account);
      if (--numberOfAccounts <= 0) {
        break;
      }
    }
  }

  /**
   * Load program onto connection
   * @param pathToProgram
   */
  async loadProgram(
    pathToProgram: string,
    payerAccount: Account,
    name: string,
  ): Promise<void> {
    const data = await fs.readFile(pathToProgram);
    const programAccount = new Account();
    await BpfLoader.load(
      this.connection,
      payerAccount,
      programAccount,
      data,
      BPF_LOADER_PROGRAM_ID,
    );
    const programId = programAccount.publicKey;
    this.programs[name] = programId;
  }

  /**
   * deploy specified contracts
   */
  async deployContracts(): Promise<void> {
    // create a new account to pay for the release
    const payerAccount = await newAccountWithLamports(this.connection, {
      lamports: 10000000000000,
      noAirdropCheck: true,
    });
    await Promise.all(
      Object.keys(PROGRAM_PATHS).map(async (programName) => {
        await this.loadProgram(
          PROGRAM_PATHS[programName],
          payerAccount,
          programName,
        );
      }),
    );
    await this.writeDeployedPrograms();
  }

  /**
   * Write the deployed programs to testDeployed.json
   */
  writeDeployedPrograms = async (): Promise<void> => {
    const programsAsStrings = Object.keys(this.programs).reduce((acc, key) => {
      acc[key] = this.programs[key].toString();
      return acc;
    }, {} as Record<string, string>);
    return fs.writeFile('testDeployed.json', JSON.stringify(programsAsStrings));
  };
}
