import { writable } from "svelte/store";

export const selected_image_files = writable();
export const qr_parse_status_message = writable('');
export const qr_code_data = writable('');