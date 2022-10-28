import { invoke } from "@tauri-apps/api";

/**
 * Parses the contents of a message into html tags.
 * @param content The text content of the message.
 * @returns The html content of the message.
 */
export function parseContent(content: string): string {

  const firstUrl = content.match(/((ftp|http|https|file):\/\/[\S]+(\b|$))(?![^<]*>|[^<>]*<)/gim);
  if (firstUrl) getEmbed(firstUrl[0]);

  // Parse urls
  content = content.replace(/((ftp|http|https|file):\/\/[\S]+(\b|$))(?![^<]*>|[^<>]*<)/gim, '<a href="$1" target="_blank">$&</a>');
  
  // Parse bold text **
  content = content.replace(/\*\*(.*?)\*\*(?![^<]*>|[^<>]*<\/)/gim, '<strong>$1</strong>');
  
  // Parse italic text *
  content = content.replace(/\*(.*?)\*(?![^<]*>|[^<>]*<\/)/gim, '<i>$1</i>');

  return content;
}

/**
 * Gets the embed information for the given msg content.
 * @param content The message content to check for urls.
 */
export async function getEmbed(content: string): Promise<{ title: string, desc: string, icon?: string, url: string, preview?: string } | undefined> {

  const url = content.match(/((ftp|http|https|file):\/\/[\S]+(\b|$))(?![^<]*>|[^<>]*<)/gim);

  if (url) {
    // Fetch the html from the url using the backend.
    let result: string = await invoke('embed', {
      url: url[0]
    });

    // If the result is empty then just return undefined.
    if (result.length === 0) return undefined;

    result = result.replace(/&#39;|&#x27;/g, "'");
    result = result.replace(/&amp;/g, "&");

    // Extract the description and title from the website html.
    const desc = /<meta.*?name="description".*?content="(.*?)".*?>|<meta.*?content="(.*?)".*?name="description".*?>/i;
    const descMatch = desc.exec(result) ?? [ "", "" ];

    const title = /<title>(.*?)<\/title>/i;
    const titleMatch = title.exec(result) ?? [ "", "" ];

    const icon = /<(link)[^>]*?href="([^>]*?)"[^>]*?icon[^>]*?>|<(link)[^>]*?icon[^>]*?href="([^>]*?)"[^>]*?>/i;
    const iconMatch = icon.exec(result) ?? [ "", "" ];
    let iconResult = iconMatch ? iconMatch[2] ? iconMatch[2] : iconMatch[4] : undefined;

    // Fix the image url which might be relative.
    if (iconResult && iconResult.length > 0 && !iconResult.includes('http')) iconResult = "http://" + new URL(url[0]).host + iconResult;

    const preview = /<(meta)[^>]*?content="([^>]*?)"[^>]*?property="og:image"[^>]*?>|<(meta)[^>]*?property="og:image"[^>]*?content="([^>]*?)"[^>]*?>/g;
    const previewMatch = preview.exec(result);
    let previewResult = previewMatch ? previewMatch[2] ? previewMatch[2] : previewMatch[4] : undefined;

    // Fix the image url which might be relative.
    if (previewResult && !previewResult.includes('http')) previewResult = "http://" + new URL(url[0]).host + previewResult;

    return { title: titleMatch[1], desc: descMatch[1], icon: iconResult, preview: previewResult, url: url[0] };
  }

  return undefined;
}