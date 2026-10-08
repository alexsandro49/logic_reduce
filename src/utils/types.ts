export interface stepStruct {
  rule_name: string,
  after_rule: string
}

export type Rule = { name: string; description: string; examples: string[]; notes: string[] }

export type TruthTableRow = {
  a: number; b: number; c: number; notA: number; notB: number
  notAOrB: number; notBOrC: number; conjunction: number; result: number
}

export type Color = { color: string, uses: string[] }