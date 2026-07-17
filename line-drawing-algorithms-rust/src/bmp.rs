use crate::framebuffer::Framebuffer;

pub fn export(fb: &Framebuffer, path: &str) {
    fb.image.export_image(path);
}
