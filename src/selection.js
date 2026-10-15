export function formatBytes(bytes) {
  const value = Number(bytes) || 0;
  if (value < 1024) return `${Math.round(value)} B`;
  const units = ["KB", "MB", "GB", "TB"];
  let size = value / 1024;
  let index = 0;
  while (size >= 1024 && index < units.length - 1) {
    size /= 1024;
    index += 1;
  }
  const digits = index >= 2 ? (size >= 100 ? 1 : 2) : size >= 10 ? 1 : 2;
  return `${size.toFixed(digits)} ${units[index]}`;
}

export function selectionOf(items, selectedIds) {
  const picked = items.filter((item) => selectedIds.has(item.id));
  const bytes = picked.reduce((sum, item) => sum + Number(item.bytes || 0), 0);
  const total = items.reduce((sum, item) => sum + Number(item.bytes || 0), 0);
  return { count: picked.length, bytes, total };
}
