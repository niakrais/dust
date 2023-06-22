export type GensRetrievedDocumentType = {
  dataSourceId: string;
  sourceUrl: string;
  documentId: string;
  timestamp: string;
  tags: string[];
  score: number;
  llm_score: number;
  chunks: {
    text: string;
    offset: number;
    score: number;
  }[];
};
