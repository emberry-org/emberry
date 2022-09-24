export interface Packet {
  type: string,
  content: any,
}

export function toPacket(event: any): Packet {

  const type: string = Object.keys(event.payload.message)[0];

  return { type, content: event.payload.message[type] };
}