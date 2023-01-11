export interface Message {
  /** Type of the message */
  type: string;

  sender: string;
  content: string;
  time: string;

  /** Is the message before this one from the same user? */
  chain: boolean;

  // embed?: {
  //   title: string;
  //   desc: string;
  //   preview?: string;
  // };
}

export interface ChatMsg {
  /* identifiers */
  sender: number;
  id: number;

  /* reference */
  reference?: number;

  /* content */
  contentType: "text" | "image" | "file";
  content: string | ImageData | FileData;
  time: number;

  /** (not stored) is the message before this one from the same user? */
  chain: boolean;
}

export interface ImageData {
  name: string;
  /** base64 string */
  data: string;
}

export interface FileData {
  name: string;
  ext: string;
}