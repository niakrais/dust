const VAULT_KINDS = ["regular", "global", "system"] as const;
export type VaultKind = (typeof VAULT_KINDS)[number];

export type VaultType = {
  name: string;
  sId: string;
  kind: VaultKind;
};
