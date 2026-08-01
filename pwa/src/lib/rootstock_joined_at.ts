export const normalizeRootstockJoinedAt = (value?: string | null) => {
  if (!value) {
    return null;
  }

  const trimmed = value.trim();
  if (!trimmed) {
    return null;
  }

  if (/^\d{4}-\d{2}-\d{2}$/.test(trimmed)) {
    return `${trimmed}T00:00:00Z`;
  }

  if (trimmed.includes("T")) {
    return trimmed;
  }

  return `${trimmed}T00:00:00Z`;
};
