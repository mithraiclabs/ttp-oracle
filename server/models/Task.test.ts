import { Task, TaskVariantKeys, TASK_LAYOUT, TASK_TAG_LAYOUT } from './Task';
import { mockPath, mockURL } from '../../testing/mockData';

describe('Task', () => {
  describe('HTTP_GET', () => {
    const urlBuffer = Buffer.from(mockURL, 'utf8');
    const tagBuffer = Buffer.alloc(TASK_TAG_LAYOUT.span);
    const taskBuffer = Buffer.concat([tagBuffer, urlBuffer]);

    it('should decode task', () => {
      const task = TASK_LAYOUT.decode(taskBuffer);
      expect(task).toEqual({
        [TaskVariantKeys[Task.HTTP_GET]]: Array.from(urlBuffer),
      });
      expect(TASK_LAYOUT.getSourceVariant(task)).toEqual(Task.HTTP_GET);
    });
  });

  describe('JSON_PARSE', () => {
    it('should decode task', () => {
      const taskBuffer = Buffer.alloc(TASK_LAYOUT.span);
      const pathBuffer = Buffer.from(mockPath, 'utf8');
      const tagBuffer = Buffer.alloc(TASK_TAG_LAYOUT.span);
      tagBuffer.writeUInt16LE(1);
      Buffer.concat([tagBuffer, pathBuffer]).copy(taskBuffer);

      const task = TASK_LAYOUT.decode(taskBuffer);
      expect(task).toEqual({
        [TaskVariantKeys[Task.JSON_PARSE]]: Array.from(pathBuffer),
      });
      expect(TASK_LAYOUT.getSourceVariant(task)).toEqual(Task.JSON_PARSE);
    });
  });

  describe('SOL_UINT_256', () => {
    it('should decode task', () => {
      const buffer = Buffer.alloc(TASK_TAG_LAYOUT.span);
      buffer.writeUInt16LE(2);

      const task = TASK_LAYOUT.decode(buffer);
      expect(task).toEqual({
        [TaskVariantKeys[Task.UINT32]]: true,
      });
      expect(TASK_LAYOUT.getSourceVariant(task)).toEqual(Task.UINT32);
    });
  });
});
