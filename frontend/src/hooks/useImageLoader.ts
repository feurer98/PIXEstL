import { useCallback, useRef, useState } from 'react';
import type { ImageInfo } from '../lib/types';

/**
 * Loads an image file (drag/drop or picker) via FileReader + Image.
 * The decoded element is kept in a ref (it does not need to trigger renders);
 * `image` holds the metadata that the UI displays. `onAspect` is called with
 * the image's height/width ratio so the caller can derive a default height.
 */
export function useImageLoader(onAspect?: (aspect: number) => void) {
  const imgElRef = useRef<HTMLImageElement | null>(null);
  const [image, setImage] = useState<ImageInfo | null>(null);
  // The original File is kept so the backend export can upload the exact bytes.
  const [imageFile, setImageFile] = useState<File | null>(null);
  // Data URL for displaying the source preview (object-fit: contain, no crop).
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);

  const loadFile = useCallback(
    (file: File | null | undefined) => {
      if (!file || !file.type.startsWith('image/')) return;
      const reader = new FileReader();
      reader.onload = (e) => {
        const dataUrl = e.target?.result as string;
        const img = new Image();
        img.onload = () => {
          imgElRef.current = img;
          onAspect?.(img.naturalHeight / img.naturalWidth);
          setImage({ name: file.name, w: img.naturalWidth, h: img.naturalHeight });
          setImageFile(file);
          setPreviewUrl(dataUrl);
        };
        img.src = dataUrl;
      };
      reader.readAsDataURL(file);
    },
    [onAspect],
  );

  return { image, imageFile, previewUrl, imgElRef, loadFile };
}
