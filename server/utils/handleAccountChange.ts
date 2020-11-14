import { Account, AccountInfo, Connection, PublicKey } from '@solana/web3.js';

import { Request } from '../models/Request';
import OracleAccount from '../models/OracleAccount';
import { reduceTasks } from './reduceTasks';
import { sendTransactionToHandleResponse } from './sendTransactionToHandleResponse';

const requestsInFlight = {};

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
  const {
    requestQueue: { requests },
  } = new OracleAccount(data);
  requests.forEach(
    async (request: Request): Promise<void> => {
      // check if the request is in flight
      if (!requestsInFlight[request.index]) {
        requestsInFlight[request.index] = request;

        const callerProgramIdBuf = Buffer.from(request.callerProgramIdBuffer);
        const callerProgramId = new PublicKey(callerProgramIdBuf);

        console.log(
          `Handling request ${
            request.index
          } for Caller Program ${callerProgramId.toString()}`,
        );
        const responseData = await reduceTasks(request.tasks);
        const requestIndexBuffer = Buffer.alloc(1);
        requestIndexBuffer.writeUInt8(request.index);
        const response = Buffer.concat([
          // Need 1 byte of padding for the Response determinant
          Buffer.alloc(1),
          responseData,
          requestIndexBuffer,
        ]);

        await sendTransactionToHandleResponse(
          connection,
          payerAccount,
          programId,
          oracleId,
          callerProgramId,
          response,
        );
        console.log('Response sent Oracle Program!');
        // After the TX has been confirmed remove the index from inflight
        delete requestsInFlight[request.index];
      }
    },
  );
};
