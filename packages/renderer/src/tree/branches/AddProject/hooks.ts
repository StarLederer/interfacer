const availableHookKeys = [
  "before_each_action",
  "after_code_download",
  "before_code_upload",
] as const;

type IHookKey = typeof availableHookKeys[number];

type IHooks = Partial<Record<IHookKey, string[]>>;

type IAction = {
  idle_name: string;
  active_name: string;
  command: string;
  user_terminated: boolean;
};

export { availableHookKeys };
export type { IHooks, IAction };
