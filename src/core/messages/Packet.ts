export interface Packet {
  type: string,
  content: any,
}

export function toPacket(event: any): Packet {
  return event.payload.message as Packet;
}