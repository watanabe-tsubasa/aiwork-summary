type Action = 
  { type: 'setAll', storeList: string[]}
  | { type: 'add', store: string }
  | { type: 'deleate', index: number}
  | { type: 'reset'}

export const initialState = null;

export const reducer = (state: null | string[], action: Action) => {
  switch (action.type) {
    case 'setAll':
      return action.storeList;

    case "add":
      return state ? [...state, action.store] : [action.store];

    case "deleate":
      if (state) {
        const deleatedState = state.filter((_, idx) => idx !== action.index);
        return deleatedState.length === 0 ? initialState : deleatedState
      } else {
        return state;
      }

    case "reset":
      return initialState;

    default:
      throw new Error('undefined action');
  }
}