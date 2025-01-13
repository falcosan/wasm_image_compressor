export interface WorkerResponse {
  success: boolean;
  data?: string;
  error?: string;
}

export interface WorkerRequest {
  inputFile: string | Uint8Array;
  inputType: string;
  outputType: string;
  compressionStrength: number;
}
