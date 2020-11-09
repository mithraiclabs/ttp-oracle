import fetch from 'node-fetch';
import _get from 'lodash/get';

import { Task, TaskVariantKeys, TASK_LAYOUT } from '../models/Task';

/**
 * Should ultimately return a buffer of the response data
 * to be send to the Caller Program
 */
export const reduceTasks = async (
  tasks: Record<string, any>[],
): Promise<Buffer> =>
  tasks.reduce(async (acc, task): Promise<string | number | Buffer> => {
    const variant: Task = TASK_LAYOUT.getSourceVariant(task);
    const value = task[TaskVariantKeys[variant]];

    // must await incase a promise was returned from one of the tasks
    const resolvedAcc = await acc;

    switch (variant) {
      case Task.HTTP_GET:
        const url = Buffer.from(value).toString().replace(/\0/g, '');
        return fetch(url);
      case Task.JSON_PARSE:
        const path = Buffer.from(value).toString().replace(/\0/g, '');
        const json = await resolvedAcc.json();
        return _get(json, path);
      case Task.UINT_128:
        const buf = Buffer.alloc(16);
        const intResponse = parseInt(resolvedAcc);
        // TODO update this to handle uint128
        buf.writeUInt32LE(intResponse);
        return buf;
      default:
        console.log('unmatched task');
        break;
    }
    return resolvedAcc;
  }, Buffer.alloc(0) as any);
