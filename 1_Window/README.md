# Window Creation
This is a bare bones implementation of a Winit Window. 

It's basically ripped directly from the documentation page here: https://docs.rs/winit/latest/winit/

Opens a window that lets us get user input and get a draw surface for a graphics library to draw to.

That's about it!


# Structure
We create a struct (App in this case) that implements the ApplicationHandler trait. It requires us to handle the resumed() and window_event() functions.

<pre>
impl ApplicationHandler for App
  └ resumed()
  └ window_event()
</pre>

The resumed() is where we create the window, since it runs on startup (and when the OS resumes our program)

The window_event() is run when we get an event from the window. We implement a simple match case to handle each one.

When these are set up, we run the event_loop from main and it will continually run these functions when necessary.

See the docs for more details.

 # Resources
    Winit: https://docs.rs/winit/latest/winit/