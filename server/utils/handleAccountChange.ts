import { Account, AccountInfo, Connection, PublicKey } from '@solana/web3.js';

import OracleRequest from '../models/Request';
import { reduceTasks } from './reduceTasks';
import { sendTransactionToHandleResponse } from './sendTransactionToHandleResponse';

// TODO need to keep track of what requests are enroute
/**
 * Factory to create the function that will handle Oracle Account data changes
 * @param connection
 * @param payerAccount
 * @param programId
 * @param oracleId
 */
export const createHandleAccountChange = (
  connection: Connection,
  payerAccount: Account,
  programId: PublicKey,
  oracleId: PublicKey,
) => async (oracleAccountInfo: AccountInfo<Buffer>): Promise<void> => {
  const { data } = oracleAccountInfo;
  const firstRequestByte = data[3];
  // check the first non instruction byte, if it's 0, there is no request in the buffer
  if (firstRequestByte === 0) {
    // no url to request, short circuit
    return;
  }
  const request = new OracleRequest(data);
  console.log(
    `Handling request for Caller Program ${request.callerProgramId.toString()}`,
  );
  const responseData = await reduceTasks(request.tasks);

  sendTransactionToHandleResponse(
    connection,
    payerAccount,
    programId,
    oracleId,
    request.callerProgramId,
    responseData,
  );
};
