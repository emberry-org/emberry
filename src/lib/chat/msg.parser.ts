/**
 * Parses the contents of a message into html tags.
 * @param content The text content of the message.
 * @returns The html content of the message.
 */
export function parseContent(content: string): string {
  // Parse urls
  content = content.replace(/((ftp|http|https|file):\/\/[\S]+(\b|$))(?![^<]*>|[^<>]*<)/gim, '<a href="$1" target="_blank">$&</a>');
  
  // Parse bold text **
  content = content.replace(/\*\*(.*?)\*\*(?![^<]*>|[^<>]*<\/)/gim, '<strong>$1</strong>');
  
  // Parse italic text *
  content = content.replace(/\*(.*?)\*(?![^<]*>|[^<>]*<\/)/gim, '<i>$1</i>');

  return content;
}