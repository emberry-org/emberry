export interface Packet {
  type: string,
  content: any,
}

export function toPacket(packet: string): Packet {
  return JSON.parse(packet) as Packet;
}