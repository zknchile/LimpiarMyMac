export function placeToKeep(before, after) {
  if (!before || before.screen !== "results" || after.screen !== "results") {
    return { sidebar: null, list: null, focusId: "" };
  }
  const sameList = before.active === after.active;
  return {
    sidebar: before.sidebar,
    list: sameList ? before.list : null,
    focusId: sameList && !after.confirm ? before.focusId : "",
  };
}
