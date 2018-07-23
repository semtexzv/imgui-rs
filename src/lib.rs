pub extern crate imgui_sys as sys;

use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_float, c_int, c_uchar, c_void};
use std::ptr;
use std::slice;
use std::str;

pub use child_frame::ChildFrame;
pub use color_editors::{
    ColorButton, ColorEdit, ColorEditMode, ColorFormat, ColorPicker, ColorPickerMode, ColorPreview,
    EditableColor,
};
pub use drag::{
    DragFloat, DragFloat2, DragFloat3, DragFloat4, DragFloatRange2, DragInt, DragInt2, DragInt3,
    DragInt4, DragIntRange2,
};
pub use fonts::{FontGlyphRange, ImFont, ImFontAtlas, ImFontConfig};
pub use input::{
    InputFloat, InputFloat2, InputFloat3, InputFloat4, InputInt, InputInt2, InputInt3, InputInt4,
    InputText, InputTextMultiline,
};
pub use menus::{Menu, MenuItem};
pub use plothistogram::PlotHistogram;
pub use plotlines::PlotLines;
pub use progressbar::ProgressBar;
pub use sliders::{
    SliderFloat, SliderFloat2, SliderFloat3, SliderFloat4, SliderInt, SliderInt2, SliderInt3,
    SliderInt4,
};
pub use string::{ImStr, ImString};
pub use style::StyleVar;
pub use sys::*;
pub use trees::{CollapsingHeader, TreeNode};
pub use window::Window;
pub use window_draw_list::{ChannelsSplit, ImColor, WindowDrawList};

mod child_frame;
mod color_editors;
mod drag;
mod fonts;
mod input;
mod menus;
mod plothistogram;
mod plotlines;
mod progressbar;
mod sliders;
mod string;
mod style;
mod trees;
mod window;
mod window_draw_list;

pub struct ImGui {
    // We need to keep ownership of the ImStr values to ensure the *const char pointer
    // lives long enough in case the ImStr contains a Cow::Owned
    ini_filename: Option<ImString>,
    log_filename: Option<ImString>,
    context: *mut sys::ImGuiContext,
}

#[macro_export]
macro_rules! im_str {
    ($e:tt) => ({
        unsafe {
          $crate::ImStr::from_utf8_with_nul_unchecked(concat!($e, "\0").as_bytes())
        }
    });
    ($e:tt, $($arg:tt)*) => ({
        unsafe {
          &$crate::ImString::from_utf8_with_nul_unchecked(
            format!(concat!($e, "\0"), $($arg)*).into_bytes())
        }
    })
}

pub struct TextureHandle<'a> {
    pub width: u32,
    pub height: u32,
    pub pixels: &'a [c_uchar],
}

pub fn get_style_color_name(color: ImGuiCol) -> &'static ImStr {
    unsafe {
        let bytes = CStr::from_ptr(sys::GetStyleColorName(color)).to_bytes_with_nul();
        ImStr::from_utf8_with_nul_unchecked(bytes)
    }
}

pub fn get_version() -> &'static str {
    unsafe {
        let bytes = CStr::from_ptr(sys::GetVersion()).to_bytes();
        str::from_utf8_unchecked(bytes)
    }
}

/// Represents one of the buttons of the mouse
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ImMouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Extra1 = 3,
    Extra2 = 4,
}

impl ImGui {
    pub fn init() -> ImGui {
        unsafe {
            let ctx = sys::CreateContext(ptr::null_mut());
            sys::SetCurrentContext(ctx);
            ImGui {
                ini_filename: None,
                log_filename: None,
                context: ctx,
            }
        }
    }
    fn io(&self) -> &sys::ImGuiIO { unsafe { &*sys::GetIO() } }
    fn io_mut(&mut self) -> &mut sys::ImGuiIO { unsafe { &mut *sys::GetIO() } }
    pub fn style(&self) -> &ImGuiStyle { unsafe { &*sys::GetStyle() } }
    pub fn style_mut(&mut self) -> &mut ImGuiStyle { unsafe { &mut *sys::GetStyle() } }
    pub fn fonts(&mut self) -> ImFontAtlas { unsafe { ImFontAtlas::from_ptr(self.io_mut().Fonts) } }
    pub fn prepare_texture<'a, F, T>(&mut self, f: F) -> T
        where
            F: FnOnce(TextureHandle<'a>) -> T,
    {
        let io = self.io();
        let mut pixels: *mut c_uchar = ptr::null_mut();
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut bytes_per_pixel: c_int = 0;
        unsafe {
            sys::BuildFontAtlas(io.Fonts, sys::RasterizerFlags::ForceAutoHint.0 as _);
            sys::ImFontAtlas_GetTexDataAsRGBA32(
                io.Fonts,
                &mut pixels,
                &mut width,
                &mut height,
                &mut bytes_per_pixel,
            );
            f(TextureHandle {
                width: width as u32,
                height: height as u32,
                pixels: slice::from_raw_parts(pixels, (width * height * bytes_per_pixel) as usize),
            })
        }
    }
    pub fn set_texture_id(&mut self, value: usize) { self.fonts().set_texture_id(value); }
    pub fn set_ini_filename(&mut self, value: Option<ImString>) {
        {
            let io = self.io_mut();
            io.IniFilename = match value {
                Some(ref x) => x.as_ptr(),
                None => ptr::null(),
            }
        }
        self.ini_filename = value;
    }
    pub fn set_log_filename(&mut self, value: Option<ImString>) {
        {
            let io = self.io_mut();
            io.LogFilename = match value {
                Some(ref x) => x.as_ptr(),
                None => ptr::null(),
            }
        }
        self.log_filename = value;
    }
    pub fn set_ini_saving_rate(&mut self, value: f32) {
        let io = self.io_mut();
        io.IniSavingRate = value;
    }
    pub fn set_font_global_scale(&mut self, value: f32) {
        let io = self.io_mut();
        io.FontGlobalScale = value;
    }
    pub fn set_mouse_double_click_time(&mut self, value: f32) {
        let io = self.io_mut();
        io.MouseDoubleClickTime = value;
    }
    pub fn set_mouse_double_click_max_dist(&mut self, value: f32) {
        let io = self.io_mut();
        io.MouseDoubleClickMaxDist = value;
    }
    pub fn set_mouse_drag_threshold(&mut self, value: f32) {
        let io = self.io_mut();
        io.MouseDragThreshold = value;
    }
    pub fn set_key_repeat_delay(&mut self, value: f32) {
        let io = self.io_mut();
        io.KeyRepeatDelay = value;
    }
    pub fn set_key_repeat_rate(&mut self, value: f32) {
        let io = self.io_mut();
        io.KeyRepeatRate = value;
    }
    pub fn display_size(&self) -> (f32, f32) {
        let io = self.io();
        (io.DisplaySize.x, io.DisplaySize.y)
    }
    pub fn display_framebuffer_scale(&self) -> (f32, f32) {
        let io = self.io();
        (io.DisplayFramebufferScale.x, io.DisplayFramebufferScale.y)
    }
    pub fn mouse_pos(&self) -> (f32, f32) {
        let io = self.io();
        (io.MousePos.x, io.MousePos.y)
    }
    pub fn set_mouse_pos(&mut self, x: f32, y: f32) {
        let io = self.io_mut();
        io.MousePos.x = x;
        io.MousePos.y = y;
    }
    /// Get mouse's position's delta between the current and the last frame.
    pub fn mouse_delta(&self) -> (f32, f32) {
        let io = self.io();
        (io.MouseDelta.x, io.MouseDelta.y)
    }
    pub fn set_mouse_down(&mut self, states: &[bool; 5]) {
        let io = self.io_mut();
        io.MouseDown = *states;
    }
    pub fn set_mouse_wheel(&mut self, value: f32) {
        let io = self.io_mut();
        io.MouseWheel = value;
    }
    /// Get mouse wheel delta
    pub fn mouse_wheel(&self) -> f32 {
        let io = self.io();
        io.MouseWheel
    }
    /// Set to `true` to have ImGui draw the cursor in software.
    /// If `false`, the OS cursor is used (default to `false`).
    pub fn set_mouse_draw_cursor(&mut self, value: bool) {
        let io = self.io_mut();
        io.MouseDrawCursor = value;
    }
    pub fn mouse_draw_cursor(&self) -> bool {
        let io = self.io();
        io.MouseDrawCursor
    }
    /// Set currently displayed cursor.
    /// Requires support in the windowing back-end if OS cursor is used.
    /// OS cursor is used if `mouse_draw_cursor` is set to `false` with
    /// [set_mouse_draw_cursor](#method.set_mouse_draw_cursor).
    pub fn set_mouse_cursor(&self, cursor: ImGuiMouseCursor) {
        unsafe {
            sys::SetMouseCursor(cursor);
        }
    }
    /// Get currently displayed cursor.
    pub fn mouse_cursor(&self) -> ImGuiMouseCursor { unsafe { sys::GetMouseCursor() } }
    /// Returns `true` if mouse is currently dragging with the `button` provided
    /// as argument.
    pub fn is_mouse_dragging(&self, button: ImMouseButton) -> bool {
        unsafe { sys::IsMouseDragging(button as c_int, -1.0) }
    }
    /// Returns `true` if the `button` provided as argument is currently down.
    pub fn is_mouse_down(&self, button: ImMouseButton) -> bool {
        unsafe { sys::IsMouseDown(button as c_int) }
    }
    /// Returns `true` if the `button` provided as argument is being clicked.
    pub fn is_mouse_clicked(&self, button: ImMouseButton) -> bool {
        unsafe { sys::IsMouseClicked(button as c_int, false) }
    }
    /// Returns `true` if the `button` provided as argument is being double-clicked.
    pub fn is_mouse_double_clicked(&self, button: ImMouseButton) -> bool {
        unsafe { sys::IsMouseDoubleClicked(button as c_int) }
    }
    /// Returns `true` if the `button` provided as argument was released
    pub fn is_mouse_released(&self, button: ImMouseButton) -> bool {
        unsafe { sys::IsMouseReleased(button as c_int) }
    }
    pub fn key_ctrl(&self) -> bool {
        let io = self.io();
        io.KeyCtrl
    }
    pub fn set_key_ctrl(&mut self, value: bool) {
        let io = self.io_mut();
        io.KeyCtrl = value;
    }
    pub fn key_shift(&self) -> bool {
        let io = self.io();
        io.KeyShift
    }
    pub fn set_key_shift(&mut self, value: bool) {
        let io = self.io_mut();
        io.KeyShift = value;
    }
    pub fn key_alt(&self) -> bool {
        let io = self.io();
        io.KeyAlt
    }
    pub fn set_key_alt(&mut self, value: bool) {
        let io = self.io_mut();
        io.KeyAlt = value;
    }
    pub fn set_key_super(&mut self, value: bool) {
        let io = self.io_mut();
        io.KeySuper = value;
    }
    pub fn set_key(&mut self, key: u8, pressed: bool) {
        let io = self.io_mut();
        io.KeysDown[key as usize] = pressed;
    }
    pub fn set_imgui_key(&mut self, key: ImGuiKey, mapping: u8) {
        let io = self.io_mut();
        io.KeyMap[key.0 as usize] = mapping as i32;
    }
    /// Map [`ImGuiKey`] values into user's key index
    pub fn get_key_index(&self, key: ImGuiKey) -> usize {
        unsafe { sys::GetKeyIndex(key) as usize }
    }
    /// Return whether specific key is being held
    ///
    /// # Example
    ///
    /// ```rust
    /// use imgui::{ImGuiKey, Ui};
    ///
    /// fn test(ui: &Ui) {
    ///     let delete_key_index = ui.imgui().get_key_index(ImGuiKey::Delete);
    ///     if ui.imgui().is_key_down(delete_key_index) {
    ///         println!("Delete is being held!");
    ///     }
    /// }
    /// ```
    pub fn is_key_down(&self, user_key_index: usize) -> bool {
        unsafe { sys::IsKeyDown(user_key_index as c_int) }
    }
    /// Return whether specific key was pressed
    pub fn is_key_pressed(&self, user_key_index: usize) -> bool {
        unsafe { sys::IsKeyPressed(user_key_index as c_int, true) }
    }
    /// Return whether specific key was released
    pub fn is_key_released(&self, user_key_index: usize) -> bool {
        unsafe { sys::IsKeyReleased(user_key_index as c_int) }
    }
    pub fn add_input_character(&mut self, character: char) {
        let mut buf = [0; 5];
        character.encode_utf8(&mut buf);
        unsafe {
            sys::ImGuiIO_AddInputCharactersUTF8(self.io_mut(), buf.as_ptr() as *const _);
        }
    }
    pub fn get_time(&self) -> f32 { unsafe { sys::GetTime() } }
    pub fn get_frame_count(&self) -> i32 { unsafe { sys::GetFrameCount() } }
    pub fn get_frame_rate(&self) -> f32 { self.io().Framerate }

    /// Processes input, and returns helper struct, that can retrieve whether this input is handled by ImGui or
    /// should be handled by application
    ///
    /// Use this, if you need to forward input events based on whether ImGui responded to them.
    /// Note: Calling this function is not necessary, ImGui will also process input on ImGui::frame`
    pub fn input_state<'ui, 'a: 'ui>(
        &'a mut self
    ) -> UiInputState<'ui> {
        unsafe { sys::internal::NewFrameUpdateHoveredWindowAndCaptureFlags(); }
        UiInputState { imgui: self }
    }

    pub fn frame<'ui, 'a: 'ui>(
        &'a mut self,
        size_points: (u32, u32),
        size_pixels: (u32, u32),
        delta_time: f32,
    ) -> Ui<'ui> {
        {
            let io = self.io_mut();
            io.DisplaySize.x = size_points.0 as c_float;
            io.DisplaySize.y = size_points.1 as c_float;
            io.DisplayFramebufferScale.x = if size_points.0 > 0 {
                size_pixels.0 as c_float / size_points.0 as c_float
            } else {
                0.0
            };
            io.DisplayFramebufferScale.y = if size_points.1 > 0 {
                size_pixels.1 as c_float / size_points.1 as c_float
            } else {
                0.0
            };
            io.DeltaTime = delta_time;
        }
        unsafe {
            sys::NewFrame();
            CURRENT_UI = Some(Ui {
                imgui: mem::transmute(self as &'a ImGui),
            });
        }
        Ui { imgui: self }
    }
}

impl Drop for ImGui {
    fn drop(&mut self) {
        unsafe {
            CURRENT_UI = None;
            sys::DestroyContext(self.context);
        }
    }
}

static mut CURRENT_UI: Option<Ui<'static>> = None;


pub struct UiInputState<'ui> {
    imgui: &'ui ImGui,
}
impl<'ui> UiInputState<'ui> {
    pub fn imgui(&self) -> &ImGui { self.imgui }
    pub fn want_capture_mouse(&self) -> bool {
        let io = self.imgui.io();
        io.WantCaptureMouse
    }
    pub fn want_capture_keyboard(&self) -> bool {
        let io = self.imgui.io();
        io.WantCaptureKeyboard
    }
}


pub struct DrawData<'a> {
    raw: &'a mut sys::ImDrawData,
}

impl<'a> DrawData<'a> {
    pub fn is_valid(&self) -> bool { self.raw.Valid }
    pub fn draw_list_count(&self) -> usize { self.raw.CmdListsCount as usize }
    pub fn total_vtx_count(&self) -> usize { self.raw.TotalVtxCount as usize }
    pub fn total_idx_count(&self) -> usize { self.raw.TotalIdxCount as usize }
    pub fn deindex_all_buffers(&mut self) {
        unsafe {
            sys::ImDrawData_DeIndexAllBuffers(self.raw);
        }
    }
    pub fn scale_clip_rects<S: Into<ImVec2>>(&mut self, sc: S) {
        unsafe {
            sys::ImDrawData_ScaleClipRects(self.raw, &sc.into() as *const _);
        }
    }
}

impl<'a> IntoIterator for &'a DrawData<'a> {
    type Item = DrawList<'a>;
    type IntoIter = DrawListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            unsafe fn cmd_lists<'a>(data: &'a sys::ImDrawData) -> &[*const sys::ImDrawList] {
                let cmd_lists: *const *const sys::ImDrawList = mem::transmute(data.CmdLists);
                slice::from_raw_parts(cmd_lists, data.CmdListsCount as usize)
            }

            DrawListIterator {
                iter: cmd_lists(&self.raw).iter(),
            }
        }
    }
}

pub struct DrawListIterator<'a> {
    iter: std::slice::Iter<'a, *const sys::ImDrawList>,
}

impl<'a> Iterator for DrawListIterator<'a> {
    type Item = DrawList<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|&ptr| unsafe {
            DrawList {
                cmd_buffer: (*ptr).CmdBuffer.as_slice(),
                idx_buffer: (*ptr).IdxBuffer.as_slice(),
                vtx_buffer: (*ptr).VtxBuffer.as_slice(),
            }
        })
    }
}

pub struct DrawList<'a> {
    pub cmd_buffer: &'a [sys::ImDrawCmd],
    pub idx_buffer: &'a [sys::ImDrawIdx],
    pub vtx_buffer: &'a [sys::ImDrawVert],
}

pub struct Ui<'ui> {
    imgui: &'ui ImGui,
}

static FMT: &'static [u8] = b"%s\0";

fn fmt_ptr() -> *const c_char { FMT.as_ptr() as *const c_char }

impl<'ui> Ui<'ui> {
    pub fn imgui(&self) -> &ImGui { self.imgui }
    pub fn want_capture_mouse(&self) -> bool {
        let io = self.imgui.io();
        io.WantCaptureMouse
    }
    pub fn want_capture_keyboard(&self) -> bool {
        let io = self.imgui.io();
        io.WantCaptureKeyboard
    }
    pub fn framerate(&self) -> f32 {
        let io = self.imgui.io();
        io.Framerate
    }
    /*
    pub fn metrics_allocs(&self) -> i32 {
        let io = self.imgui.io();
        io.MetricsAllocs
    }
    */
    pub fn metrics_render_vertices(&self) -> i32 {
        let io = self.imgui.io();
        io.MetricsRenderVertices
    }
    pub fn metrics_render_indices(&self) -> i32 {
        let io = self.imgui.io();
        io.MetricsRenderIndices
    }
    pub fn metrics_active_windows(&self) -> i32 {
        let io = self.imgui.io();
        io.MetricsActiveWindows
    }
    pub fn render<F, E>(self, f: F) -> Result<(), E>
        where
            F: FnOnce(&Ui, DrawData) -> Result<(), E>,
    {
        unsafe {
            sys::Render();

            let draw_data = DrawData {
                raw: &mut *sys::GetDrawData(),
            };
            f(&self, draw_data)?;
            CURRENT_UI = None;
        }
        Ok(())
    }
    pub fn show_user_guide(&self) { unsafe { sys::ShowUserGuide() }; }
    pub fn show_default_style_editor(&self) { unsafe { sys::ShowStyleEditor(ptr::null_mut()) }; }
    pub fn show_style_editor<'p>(&self, style: &'p mut ImGuiStyle) {
        unsafe {
            sys::ShowStyleEditor(style as *mut ImGuiStyle);
        }
    }
    #[deprecated(
    since = "0.0.19",
    note = "please use show_demo_window instead"
    )]
    pub fn show_test_window(&self, opened: &mut bool) { self.show_demo_window(opened) }
    pub fn show_demo_window(&self, opened: &mut bool) {
        unsafe {
            sys::ShowDemoWindow(opened);
        }
    }
    pub fn show_metrics_window(&self, opened: &mut bool) {
        unsafe {
            sys::ShowMetricsWindow(opened);
        }
    }
}

impl<'a> Ui<'a> {
    pub unsafe fn current_ui() -> Option<&'a Ui<'a>> { CURRENT_UI.as_ref() }
}

// Window
impl<'ui> Ui<'ui> {
    pub fn window<'p>(&self, name: &'p ImStr) -> Window<'ui, 'p> { Window::new(self, name) }
    /// Get current window's size in pixels
    pub fn get_window_size(&self) -> (f32, f32) { unsafe { sys::GetWindowSize().into() } }
}

// Layout
impl<'ui> Ui<'ui> {
    /// Pushes a value to the item width stack.
    pub fn push_item_width(&self, width: f32) { unsafe { sys::PushItemWidth(width) } }

    /// Pops a value from the item width stack.
    ///
    /// # Aborts
    /// The current process is aborted if the item width stack is empty.
    pub fn pop_item_width(&self) { unsafe { sys::PopItemWidth() } }

    /// Runs a function after temporarily pushing a value to the item width stack.
    pub fn with_item_width<F>(&self, width: f32, f: F)
        where
            F: FnOnce(),
    {
        self.push_item_width(width);
        f();
        self.pop_item_width();
    }

    pub fn separator(&self) { unsafe { sys::Separator() }; }
    pub fn new_line(&self) { unsafe { sys::NewLine() } }
    pub fn same_line(&self, pos_x: f32) { unsafe { sys::SameLine(pos_x, -1.0f32) } }
    pub fn same_line_spacing(&self, pos_x: f32, spacing_w: f32) {
        unsafe { sys::SameLine(pos_x, spacing_w) }
    }
    pub fn spacing(&self) { unsafe { sys::Spacing() }; }

    pub fn columns<'p>(&self, count: i32, id: &'p ImStr, border: bool) {
        unsafe { sys::Columns(count, id.as_ptr(), border) }
    }

    pub fn next_column(&self) { unsafe { sys::NextColumn() } }

    pub fn get_column_index(&self) -> i32 { unsafe { sys::GetColumnIndex() } }

    pub fn get_column_offset(&self, column_index: i32) -> f32 {
        unsafe { sys::GetColumnOffset(column_index) }
    }

    pub fn set_column_offset(&self, column_index: i32, offset_x: f32) {
        unsafe { sys::SetColumnOffset(column_index, offset_x) }
    }

    pub fn get_column_width(&self, column_index: i32) -> f32 {
        unsafe { sys::GetColumnWidth(column_index) }
    }

    pub fn get_columns_count(&self) -> i32 { unsafe { sys::GetColumnsCount() } }

    /// Fill a space of `size` in pixels with nothing on the current window.
    /// Can be used to move the cursor on the window.
    pub fn dummy<S: Into<ImVec2>>(&self, size: S) {
        let size = size.into();
        unsafe { sys::Dummy(&size) }
    }

    /// Get cursor position on the screen, in screen coordinates.
    /// This sets the point on which the next widget will be drawn.
    ///
    /// This is especially useful for drawing, as the drawing API uses
    /// screen coordiantes.
    pub fn get_cursor_screen_pos(&self) -> (f32, f32) {
        unsafe { sys::GetCursorScreenPos().into() }
    }

    /// Set cursor position on the screen, in screen coordinates.
    /// This sets the point on which the next widget will be drawn.
    pub fn set_cursor_screen_pos<P: Into<ImVec2>>(&self, pos: P) {
        unsafe { sys::SetCursorScreenPos(&pos.into() as *const _) }
    }

    /// Get cursor position on the screen, in window coordinates.
    pub fn get_cursor_pos(&self) -> (f32, f32) { unsafe { sys::GetCursorPos().into() } }

    /// Set cursor position on the screen, in window coordinates.
    /// This sets the point on which the next widget will be drawn.
    pub fn set_cursor_pos<P: Into<ImVec2>>(&self, pos: P) {
        unsafe { sys::SetCursorPos(&pos.into() as *const _) }
    }

    pub fn scroll_max_x(&self) -> f32 {
        unsafe { sys::GetScrollMaxX() }
    }

    pub fn scroll_to_x(&self, v: f32) {
        unsafe { sys::SetScrollX(v); }
    }

    pub fn scroll_to_y(&self, v: f32) {
        unsafe { sys::SetScrollY(v); }
    }

    /// Get available space left between the cursor and the edges of the current
    /// window.
    pub fn get_content_region_avail(&self) -> (f32, f32) {
        unsafe { sys::GetContentRegionAvail().into() }
    }
}

pub enum ImId<'a> {
    Int(i32),
    Str(&'a str),
    Ptr(*const c_void),
}

impl From<i32> for ImId<'static> {
    fn from(i: i32) -> Self { ImId::Int(i) }
}

impl<'a, T: ?Sized + AsRef<str>> From<&'a T> for ImId<'a> {
    fn from(s: &'a T) -> Self { ImId::Str(s.as_ref()) }
}

impl<T> From<*const T> for ImId<'static> {
    fn from(p: *const T) -> Self { ImId::Ptr(p as *const c_void) }
}

impl<T> From<*mut T> for ImId<'static> {
    fn from(p: *mut T) -> Self { ImId::Ptr(p as *const T as *const c_void) }
}

// ID scopes
impl<'ui> Ui<'ui> {
    /// Pushes an identifier to the ID stack.
    pub fn push_id<'a, I: Into<ImId<'a>>>(&self, id: I) {
        let id = id.into();

        unsafe {
            match id {
                ImId::Int(i) => {
                    sys::PushID3(i);
                }
                ImId::Str(s) => {
                    let start = s.as_ptr() as *const c_char;
                    let end = start.offset(s.len() as isize);
                    sys::PushID1(start, end);
                }
                ImId::Ptr(p) => {
                    sys::PushID2(p as *const c_void);
                }
            }
        }
    }

    /// Pops an identifier from the ID stack.
    ///
    /// # Aborts
    /// The current process is aborted if the ID stack is empty.
    pub fn pop_id(&self) { unsafe { sys::PopID() }; }

    /// Runs a function after temporarily pushing a value to the ID stack.
    pub fn with_id<'a, F, I>(&self, id: I, f: F)
        where
            F: FnOnce(),
            I: Into<ImId<'a>>,
    {
        self.push_id(id);
        f();
        self.pop_id();
    }
}

// Widgets
impl<'ui> Ui<'ui> {
    pub fn text<T: AsRef<str>>(&self, text: T) {
        let s = text.as_ref();
        unsafe {
            let start = s.as_ptr();
            let end = start.offset(s.len() as isize);
            sys::TextUnformatted(start as *const c_char, end as *const c_char);
        }
    }
    pub fn text_colored<'p, A>(&self, col: A, text: &'p ImStr)
        where
            A: Into<ImVec4>,
    {
        unsafe {
            sys::TextColored(&col.into() as *const _, fmt_ptr(), text.as_ptr());
        }
    }
    pub fn text_disabled<'p>(&self, text: &'p ImStr) {
        unsafe {
            sys::TextDisabled(fmt_ptr(), text.as_ptr());
        }
    }
    pub fn text_wrapped<'p>(&self, text: &'p ImStr) {
        unsafe {
            sys::TextWrapped(fmt_ptr(), text.as_ptr());
        }
    }
    pub fn label_text<'p>(&self, label: &'p ImStr, text: &'p ImStr) {
        unsafe {
            sys::LabelText(label.as_ptr(), fmt_ptr(), text.as_ptr());
        }
    }
    pub fn bullet(&self) {
        unsafe {
            sys::Bullet();
        }
    }
    pub fn bullet_text<'p>(&self, text: &'p ImStr) {
        unsafe {
            sys::BulletText(fmt_ptr(), text.as_ptr());
        }
    }
    pub fn button<'p, S: Into<ImVec2>>(&self, label: &'p ImStr, size: S) -> bool {
        unsafe { sys::Button(label.as_ptr(), &size.into() as *const _) }
    }
    pub fn small_button<'p>(&self, label: &'p ImStr) -> bool {
        unsafe { sys::SmallButton(label.as_ptr()) }
    }
    /// Make a invisible event. Can be used to conveniently catch events when
    /// mouse hovers or click the area covered by this invisible button.
    pub fn invisible_button<'p, S: Into<ImVec2>>(&self, label: &'p ImStr, size: S) -> bool {
        unsafe { sys::InvisibleButton(label.as_ptr(), &size.into() as *const _) }
    }
    pub fn checkbox<'p>(&self, label: &'p ImStr, value: &'p mut bool) -> bool {
        unsafe { sys::Checkbox(label.as_ptr(), value) }
    }
}

// Widgets: Input
impl<'ui> Ui<'ui> {
    pub fn input_text<'p>(&self, label: &'p ImStr, buf: &'p mut ImString) -> InputText<'ui, 'p> {
        InputText::new(self, label, buf)
    }
    pub fn input_text_multiline<'p, S: Into<ImVec2>>(
        &self,
        label: &'p ImStr,
        buf: &'p mut ImString,
        size: S,
    ) -> InputTextMultiline<'ui, 'p> {
        InputTextMultiline::new(self, label, buf, size.into())
    }
    pub fn input_float<'p>(&self, label: &'p ImStr, value: &'p mut f32) -> InputFloat<'ui, 'p> {
        InputFloat::new(self, label, value)
    }
    pub fn input_float2<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 2],
    ) -> InputFloat2<'ui, 'p> {
        InputFloat2::new(self, label, value)
    }
    pub fn input_float3<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 3],
    ) -> InputFloat3<'ui, 'p> {
        InputFloat3::new(self, label, value)
    }
    pub fn input_float4<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 4],
    ) -> InputFloat4<'ui, 'p> {
        InputFloat4::new(self, label, value)
    }
    pub fn input_int<'p>(&self, label: &'p ImStr, value: &'p mut i32) -> InputInt<'ui, 'p> {
        InputInt::new(self, label, value)
    }
    pub fn input_int2<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 2]) -> InputInt2<'ui, 'p> {
        InputInt2::new(self, label, value)
    }
    pub fn input_int3<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 3]) -> InputInt3<'ui, 'p> {
        InputInt3::new(self, label, value)
    }
    pub fn input_int4<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 4]) -> InputInt4<'ui, 'p> {
        InputInt4::new(self, label, value)
    }
}

// Widgets: Drag
impl<'ui> Ui<'ui> {
    pub fn drag_float<'p>(&self, label: &'p ImStr, value: &'p mut f32) -> DragFloat<'ui, 'p> {
        DragFloat::new(self, label, value)
    }
    pub fn drag_float2<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 2],
    ) -> DragFloat2<'ui, 'p> {
        DragFloat2::new(self, label, value)
    }
    pub fn drag_float3<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 3],
    ) -> DragFloat3<'ui, 'p> {
        DragFloat3::new(self, label, value)
    }
    pub fn drag_float4<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 4],
    ) -> DragFloat4<'ui, 'p> {
        DragFloat4::new(self, label, value)
    }
    pub fn drag_float_range2<'p>(
        &self,
        label: &'p ImStr,
        current_min: &'p mut f32,
        current_max: &'p mut f32,
    ) -> DragFloatRange2<'ui, 'p> {
        DragFloatRange2::new(self, label, current_min, current_max)
    }
    pub fn drag_int<'p>(&self, label: &'p ImStr, value: &'p mut i32) -> DragInt<'ui, 'p> {
        DragInt::new(self, label, value)
    }
    pub fn drag_int2<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 2]) -> DragInt2<'ui, 'p> {
        DragInt2::new(self, label, value)
    }
    pub fn drag_int3<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 3]) -> DragInt3<'ui, 'p> {
        DragInt3::new(self, label, value)
    }
    pub fn drag_int4<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 4]) -> DragInt4<'ui, 'p> {
        DragInt4::new(self, label, value)
    }
    pub fn drag_int_range2<'p>(
        &self,
        label: &'p ImStr,
        current_min: &'p mut i32,
        current_max: &'p mut i32,
    ) -> DragIntRange2<'ui, 'p> {
        DragIntRange2::new(self, label, current_min, current_max)
    }
}

// Widgets: Sliders
impl<'ui> Ui<'ui> {
    pub fn slider_float<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut f32,
        min: f32,
        max: f32,
    ) -> SliderFloat<'ui, 'p> {
        SliderFloat::new(self, label, value, min, max)
    }
    pub fn slider_float2<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 2],
        min: f32,
        max: f32,
    ) -> SliderFloat2<'ui, 'p> {
        SliderFloat2::new(self, label, value, min, max)
    }
    pub fn slider_float3<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 3],
        min: f32,
        max: f32,
    ) -> SliderFloat3<'ui, 'p> {
        SliderFloat3::new(self, label, value, min, max)
    }
    pub fn slider_float4<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 4],
        min: f32,
        max: f32,
    ) -> SliderFloat4<'ui, 'p> {
        SliderFloat4::new(self, label, value, min, max)
    }
    pub fn slider_int<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut i32,
        min: i32,
        max: i32,
    ) -> SliderInt<'ui, 'p> {
        SliderInt::new(self, label, value, min, max)
    }
    pub fn slider_int2<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [i32; 2],
        min: i32,
        max: i32,
    ) -> SliderInt2<'ui, 'p> {
        SliderInt2::new(self, label, value, min, max)
    }
    pub fn slider_int3<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [i32; 3],
        min: i32,
        max: i32,
    ) -> SliderInt3<'ui, 'p> {
        SliderInt3::new(self, label, value, min, max)
    }
    pub fn slider_int4<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [i32; 4],
        min: i32,
        max: i32,
    ) -> SliderInt4<'ui, 'p> {
        SliderInt4::new(self, label, value, min, max)
    }
}

// Widgets: Color Editor/Picker
impl<'ui> Ui<'ui> {
    /// Constructs a new color editor builder.
    pub fn color_edit<'p, V: Into<EditableColor<'p>>>(
        &self,
        label: &'p ImStr,
        value: V,
    ) -> ColorEdit<'ui, 'p> {
        ColorEdit::new(self, label, value.into())
    }
    /// Constructs a new color picker builder.
    pub fn color_picker<'p, V: Into<EditableColor<'p>>>(
        &self,
        label: &'p ImStr,
        value: V,
    ) -> ColorPicker<'ui, 'p> {
        ColorPicker::new(self, label, value.into())
    }
    /// Constructs a new color button builder.
    pub fn color_button<'p, C: Into<ImVec4>>(
        &self,
        desc_id: &'p ImStr,
        color: C,
    ) -> ColorButton<'ui, 'p> {
        ColorButton::new(self, desc_id, color.into())
    }
    /// Initialize current options (generally on application startup) if you want to select a
    /// default format, picker type, etc. Users will be able to change many settings, unless you
    /// use .options(false) in your widget builders.
    pub fn set_color_edit_options(&self, flags: ImGuiColorEditFlags) {
        unsafe {
            sys::SetColorEditOptions(flags);
        }
    }
}

// Widgets: Trees
impl<'ui> Ui<'ui> {
    pub fn tree_node<'p>(&self, id: &'p ImStr) -> TreeNode<'ui, 'p> { TreeNode::new(self, id) }
    pub fn collapsing_header<'p>(&self, label: &'p ImStr) -> CollapsingHeader<'ui, 'p> {
        CollapsingHeader::new(self, label)
    }
}

// Widgets: Selectable / Lists
impl<'ui> Ui<'ui> {
    pub fn selectable<'p, S: Into<ImVec2>>(
        &self,
        label: &'p ImStr,
        selected: bool,
        flags: ImGuiSelectableFlags,
        size: S,
    ) -> bool {
        unsafe { sys::Selectable(label.as_ptr(), selected, flags, &size.into() as *const _) }
    }
}

/// # Tooltips
impl<'ui> Ui<'ui> {
    /// Construct a tooltip window that can have any kind of content.
    ///
    /// Typically used with `Ui::is_item_hovered()` or some other conditional check.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate imgui;
    /// # use imgui::*;
    /// fn user_interface(ui: &Ui) {
    ///     ui.text("Hover over me");
    ///     if ui.is_item_hovered() {
    ///         ui.tooltip(|| {
    ///             ui.text_colored((1.0, 0.0, 0.0, 1.0), im_str!("I'm red!"));
    ///         });
    ///     }
    /// }
    /// # fn main() {
    /// # }
    /// ```
    pub fn tooltip<F: FnOnce()>(&self, f: F) {
        unsafe { sys::BeginTooltip() };
        f();
        unsafe { sys::EndTooltip() };
    }
    /// Construct a tooltip window with simple text content.
    ///
    /// Typically used with `Ui::is_item_hovered()` or some other conditional check.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate imgui;
    /// # use imgui::*;
    /// fn user_interface(ui: &Ui) {
    ///     ui.text("Hover over me");
    ///     if ui.is_item_hovered() {
    ///         ui.tooltip_text("I'm a tooltip!");
    ///     }
    /// }
    /// # fn main() {
    /// # }
    /// ```
    pub fn tooltip_text<T: AsRef<str>>(&self, text: T) { self.tooltip(|| self.text(text)); }
}

// Widgets: Menus
impl<'ui> Ui<'ui> {
    pub fn main_menu_bar<F>(&self, f: F)
        where
            F: FnOnce(),
    {
        let render = unsafe { sys::BeginMainMenuBar() };
        if render {
            f();
            unsafe { sys::EndMainMenuBar() };
        }
    }
    pub fn menu_bar<F>(&self, f: F)
        where
            F: FnOnce(),
    {
        let render = unsafe { sys::BeginMenuBar() };
        if render {
            f();
            unsafe { sys::EndMenuBar() };
        }
    }
    pub fn menu<'p>(&self, label: &'p ImStr) -> Menu<'ui, 'p> { Menu::new(self, label) }
    pub fn menu_item<'p>(&self, label: &'p ImStr) -> MenuItem<'ui, 'p> {
        MenuItem::new(self, label)
    }
}

// Widgets: Popups
impl<'ui> Ui<'ui> {
    pub fn open_popup<'p>(&self, str_id: &'p ImStr) { unsafe { sys::OpenPopup(str_id.as_ptr()) }; }
    pub fn popup<'p, F>(&self, str_id: &'p ImStr, f: F)
        where
            F: FnOnce(),
    {
        let render = unsafe { sys::BeginPopup(str_id.as_ptr(), sys::ImGuiWindowFlags::None) };
        if render {
            f();
            unsafe { sys::EndPopup() };
        }
    }
    pub fn close_current_popup(&self) { unsafe { sys::CloseCurrentPopup() }; }
}

// Widgets: Combos
impl<'ui> Ui<'ui> {
    pub fn combo<'p>(
        &self,
        label: &'p ImStr,
        current_item: &mut i32,
        items: &'p [&'p ImStr],
        height_in_items: i32,
    ) -> bool {
        let items_inner: Vec<*const c_char> = items.into_iter().map(|item| item.as_ptr()).collect();
        unsafe {
            sys::Combo(
                label.as_ptr(),
                current_item,
                items_inner.as_ptr() as *mut *const c_char,
                items_inner.len() as i32,
                height_in_items,
            )
        }
    }
}

// Widgets: ListBox
impl<'ui> Ui<'ui> {
    pub fn list_box<'p>(
        &self,
        label: &'p ImStr,
        current_item: &mut i32,
        items: &'p [&'p ImStr],
        height_in_items: i32,
    ) -> bool {
        let items_inner: Vec<*const c_char> = items.into_iter().map(|item| item.as_ptr()).collect();
        unsafe {
            sys::ListBox(
                label.as_ptr(),
                current_item,
                items_inner.as_ptr() as *mut *const c_char,
                items_inner.len() as i32,
                height_in_items,
            )
        }
    }
}

// Widgets: Radio
impl<'ui> Ui<'ui> {
    /// Creates a radio button for selecting an integer value.
    /// Returns true if pressed.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// # let mut selected_radio_value = 2;
    /// ui.radio_button(im_str!("Item 1"), &mut selected_radio_value, 1);
    /// ui.radio_button(im_str!("Item 2"), &mut selected_radio_value, 2);
    /// ui.radio_button(im_str!("Item 3"), &mut selected_radio_value, 3);
    /// ```
    pub fn radio_button<'p>(&self, label: &'p ImStr, value: &'p mut i32, wanted: i32) -> bool {
        unsafe { sys::RadioButton1(label.as_ptr(), value, wanted) }
    }

    /// Creates a radio button that shows as selected if the given value is true.
    /// Returns true if pressed.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// # let mut radio_button_test = "cats".to_string();
    /// if ui.radio_button_bool(im_str!("Cats"), radio_button_test == "cats") {
    ///     radio_button_test = "cats".to_string();
    /// }
    /// if ui.radio_button_bool(im_str!("Dogs"), radio_button_test == "dogs") {
    ///     radio_button_test = "dogs".to_string();
    /// }
    /// ```
    pub fn radio_button_bool<'p>(&self, label: &'p ImStr, value: bool) -> bool {
        unsafe { sys::RadioButton(label.as_ptr(), value) }
    }
}

impl<'ui> Ui<'ui> {
    pub fn plot_lines<'p>(&self, label: &'p ImStr, values: &'p [f32]) -> PlotLines<'ui, 'p> {
        PlotLines::new(self, label, values)
    }
}

impl<'ui> Ui<'ui> {
    pub fn plot_histogram<'p>(
        &self,
        label: &'p ImStr,
        values: &'p [f32],
    ) -> PlotHistogram<'ui, 'p> {
        PlotHistogram::new(self, label, values)
    }
}

impl<'ui> Ui<'ui> {
    /// Calculate the size required for a given text string.
    ///
    /// hide_text_after_double_hash allows the user to insert comments into their text, using a double hash-tag prefix.
    /// This is a feature of imgui.
    ///
    /// wrap_width allows you to request a width at which to wrap the text to a newline for the calculation.
    pub fn calc_text_size(
        &self,
        text: &ImStr,
        hide_text_after_double_hash: bool,
        wrap_width: f32,
    ) -> ImVec2 {
        unsafe {
            sys::CalcTextSize(
                text.as_ptr(),
                std::ptr::null(),
                hide_text_after_double_hash,
                wrap_width,
            ).into()
        }
    }
}

impl<'ui> Ui<'ui> {
    /// Get height of a line of previously drawn text item
    pub fn get_text_line_height_with_spacing(&self) -> f32 {
        unsafe { sys::GetTextLineHeightWithSpacing() }
    }
    /// Get previously drawn item's size
    pub fn get_item_rect_size(&self) -> (f32, f32) { unsafe { sys::GetItemRectSize().into() } }
}

impl<'ui> Ui<'ui> {
    /// Creates a progress bar. Fraction is the progress level with 0.0 = 0% and 1.0 = 100%.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// ui.progress_bar(0.6)
    ///     .size((100.0, 12.0))
    ///     .overlay_text(im_str!("Progress!"))
    ///     .build();
    /// ```
    pub fn progress_bar<'p>(&self, fraction: f32) -> ProgressBar<'ui, 'p> {
        ProgressBar::new(self, fraction)
    }
}

impl<'ui> Ui<'ui> {
    /// Creates a child frame. Size is size of child_frame within parent window.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// ui.window(im_str!("ChatWindow"))
    ///     .title_bar(true)
    ///     .scrollable(false)
    ///     .build(|| {
    ///         ui.separator();
    ///
    ///         ui.child_frame(im_str!("child frame"), (400.0, 100.0))
    ///             .show_borders(true)
    ///             .always_show_vertical_scroll_bar(true)
    ///             .build(|| {
    ///                 ui.text_colored((1.0, 0.0, 0.0, 1.0), im_str!("hello mate!"));
    ///             });
    /// });
    pub fn child_frame<'p, S: Into<ImVec2>>(
        &self,
        name: &'p ImStr,
        size: S,
    ) -> ChildFrame<'ui, 'p> {
        ChildFrame::new(self, name, size.into())
    }
}

impl<'ui> Ui<'ui> {
    /// Runs a function after temporarily pushing a value to the style stack.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// ui.with_style_var(StyleVar::Alpha(0.2), || {
    ///     ui.text(im_str!("AB"));
    /// });
    /// ```
    pub fn with_style_var<F: FnOnce()>(&self, style_var: StyleVar, f: F) {
        self.push_style_var(style_var);
        f();
        unsafe { sys::PopStyleVar(1) }
    }

    /// Runs a function after temporarily pushing an array of values into the stack. Supporting
    /// multiple is also easy since you can freely mix and match them in a safe manner.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// # let styles = [StyleVar::Alpha(0.2), StyleVar::WindowPadding(ImVec2::new(1.0, 1.0))];
    /// ui.with_style_vars(&styles, || {
    ///     ui.text(im_str!("A"));
    ///     ui.text(im_str!("B"));
    ///     ui.text(im_str!("C"));
    ///     ui.text(im_str!("D"));
    /// });
    /// ```
    pub fn with_style_vars<F: FnOnce()>(&self, style_vars: &[StyleVar], f: F) {
        for &style_var in style_vars {
            self.push_style_var(style_var);
        }
        f();
        unsafe { sys::PopStyleVar(style_vars.len() as i32) };
    }

    #[inline]
    fn push_style_var(&self, style_var: StyleVar) {
        use sys::ImGuiStyleVar;
        use sys::{PushStyleVar, PushStyleVar1};
        use StyleVar::*;
        match style_var {
            Alpha(v) => unsafe { PushStyleVar(ImGuiStyleVar::Alpha, v) },
            WindowPadding(v) => unsafe {
                PushStyleVar1(ImGuiStyleVar::WindowPadding, &v as *const _)
            },
            WindowRounding(v) => unsafe { PushStyleVar(ImGuiStyleVar::WindowRounding, v) },
            WindowBorderSize(v) => unsafe { PushStyleVar(ImGuiStyleVar::WindowBorderSize, v) },
            WindowMinSize(v) => unsafe {
                PushStyleVar1(ImGuiStyleVar::WindowMinSize, &v as *const _)
            },
            ChildRounding(v) => unsafe { PushStyleVar(ImGuiStyleVar::ChildRounding, v) },
            ChildBorderSize(v) => unsafe { PushStyleVar(ImGuiStyleVar::ChildBorderSize, v) },
            PopupRounding(v) => unsafe { PushStyleVar(ImGuiStyleVar::PopupRounding, v) },
            PopupBorderSize(v) => unsafe { PushStyleVar(ImGuiStyleVar::PopupBorderSize, v) },
            FramePadding(v) => unsafe {
                PushStyleVar1(ImGuiStyleVar::FramePadding, &v as *const _)
            },
            FrameRounding(v) => unsafe { PushStyleVar(ImGuiStyleVar::FrameRounding, v) },
            FrameBorderSize(v) => unsafe { PushStyleVar(ImGuiStyleVar::FrameBorderSize, v) },
            ItemSpacing(v) => unsafe { PushStyleVar1(ImGuiStyleVar::ItemSpacing, &v as *const _) },
            ItemInnerSpacing(v) => unsafe {
                PushStyleVar1(ImGuiStyleVar::ItemInnerSpacing, &v as *const _)
            },
            IndentSpacing(v) => unsafe { PushStyleVar(ImGuiStyleVar::IndentSpacing, v) },
            GrabMinSize(v) => unsafe { PushStyleVar(ImGuiStyleVar::GrabMinSize, v) },
            ButtonTextAlign(v) => unsafe {
                PushStyleVar1(ImGuiStyleVar::ButtonTextAlign, &v as *const _)
            },
        }
    }
}

impl<'ui> Ui<'ui> {
    /// Runs a function after temporarily pushing a value to the color stack.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// ui.with_color_var(ImGuiCol::Text, (1.0, 0.0, 0.0, 1.0), || {
    ///     ui.text_wrapped(im_str!("AB"));
    /// });
    /// ```
    pub fn with_color_var<F: FnOnce(), C: Into<ImVec4> + Copy>(
        &self,
        var: ImGuiCol,
        color: C,
        f: F,
    ) {
        unsafe {
            sys::PushStyleColor1(var, &color.into() as *const _);
        }
        f();
        unsafe {
            sys::PopStyleColor(1);
        }
    }

    /// Runs a function after temporarily pushing an array of values to the color stack.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// let red = (1.0, 0.0, 0.0, 1.0);
    /// let green = (0.0, 1.0, 0.0, 1.0);
    /// # let vars = [(ImGuiCol::Text, red), (ImGuiCol::TextDisabled, green)];
    /// ui.with_color_vars(&vars, || {
    ///     ui.text_wrapped(im_str!("AB"));
    /// });
    /// ```
    pub fn with_color_vars<F: FnOnce(), C: Into<ImVec4> + Copy>(
        &self,
        color_vars: &[(ImGuiCol, C)],
        f: F,
    ) {
        for &(color_var, color) in color_vars {
            unsafe {
                sys::PushStyleColor1(color_var, &color.into() as *const _);
            }
        }
        f();
        unsafe { sys::PopStyleColor(color_vars.len() as i32) };
    }
}

impl<'ui> Ui<'ui> {
    /// Runs a function after temporarily pushing an array of values to the
    /// style and color stack.
    pub fn with_style_and_color_vars<F, C>(
        &self,
        style_vars: &[StyleVar],
        color_vars: &[(ImGuiCol, C)],
        f: F,
    ) where
        F: FnOnce(),
        C: Into<ImVec4> + Copy,
    {
        self.with_style_vars(style_vars, || {
            self.with_color_vars(color_vars, f);
        });
    }
}

/// # Utilities
impl<'ui> Ui<'ui> {
    /// Returns `true` if the last item is being hovered by the mouse.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate imgui;
    /// # use imgui::*;
    /// fn user_interface(ui: &Ui) {
    ///     ui.text("Hover over me");
    ///     let is_hover_over_me_text_hovered = ui.is_item_hovered();
    /// }
    /// # fn main() {
    /// # }
    /// ```
    pub fn is_item_hovered(&self) -> bool { unsafe { sys::IsItemHovered(ImGuiHoveredFlags::None) } }

    /// Return `true` if the current window is being hovered by the mouse.
    pub fn is_window_hovered(&self) -> bool {
        unsafe { sys::IsWindowHovered(ImGuiHoveredFlags::None) }
    }

    /// Returns `true` if the last item is being active.
    pub fn is_item_active(&self) -> bool { unsafe { sys::IsItemActive() } }

    /// Group items together as a single item.
    ///
    /// May be useful to handle the same mouse event on a group of items, for example.
    pub fn group<F: FnOnce()>(&self, f: F) {
        unsafe {
            sys::BeginGroup();
        }
        f();
        unsafe {
            sys::EndGroup();
        }
    }
}

/// # Draw list for custom drawing
impl<'ui> Ui<'ui> {
    /// Get access to drawing API
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use imgui::*;
    /// fn custom_draw(ui: &Ui) {
    ///     let draw_list = ui.get_window_draw_list();
    ///     // Draw a line
    ///     const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
    ///     draw_list.add_line([100.0, 100.0], [200.0, 200.0], WHITE).build();
    ///     // Continue drawing ...
    /// }
    /// ```
    ///
    /// This function will panic if several instances of [`WindowDrawList`]
    /// coexist. Before a new instance is got, a previous instance should be
    /// dropped.
    ///
    /// ```rust
    /// # use imgui::*;
    /// fn custom_draw(ui: &Ui) {
    ///     let draw_list = ui.get_window_draw_list();
    ///     // Draw something...
    ///
    ///     // This second call will panic!
    ///     let draw_list = ui.get_window_draw_list();
    /// }
    /// ```
    pub fn get_window_draw_list(&'ui self) -> WindowDrawList<'ui> { WindowDrawList::new(self) }
}
