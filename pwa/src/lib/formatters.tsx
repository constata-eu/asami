export const formatTxHash = (hash) => `${hash.substring(0,8)}…${hash.substring(60)}`

export const formatAddress = (addr) => `${addr.substring(0,8)}…${addr.substring(36)}`
