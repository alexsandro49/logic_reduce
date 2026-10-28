export interface stepStruct {
  rule_name: string,
  after_rule: string
};

export type Rule = { name: string; description: string; examples: string[]; 
                                                                      notes: string[] };
export type Color = { color: string, tags: string[] };

export type TruthTableRows = number[];

export type EquivalenceResult = {
  equivalent: boolean;
  counterexample: {
    inputs: Record<string, boolean>;
    original: boolean;
    candidate: boolean;
  } | null;
};

export type ManualCompletion = {
  complete: boolean;
  next_rule: string | null;
};

export type ManualStepValidation = {
  equivalence: EquivalenceResult;
  law_valid: boolean;
  hint: string;
};

export type TruthTableRow = {
  inputs: number[];
  outputs: number[];
}

export type TruthTable = {
    inputs: string[],
    columns: string[],
    rows: TruthTableRow[],
    total_rows: number,
}

export type ReductionStep = { value: string; rule: string }

export type ReductionStepAutomatic = { 
  title: string;
  expression: string;
  description: string;
}

export type StepStruct = {
  rule_name: string,
  after_rule: string
};

