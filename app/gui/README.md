# XMODITS GUI Application
The code is slightly messy.
## Key areas
* [Styling](./src/gui/style/)
* [Embedding External Fonts](./src/core/font.rs)
* [Font Icons](./src/gui/icons.rs)
* [Subscription](./src/core/xmodits.rs) go to Subscription for more detail.
* [Configuration](./src/core/cfg.rs)
* [Views](./src/gui/views/)

### Layout & Views
Refer to [Screenshots](#screenshots)
* [Main Application](./src/gui/mod.rs)
* [Sample Naming View](./src/gui/views/config_name.rs)
* [Ripping Configuration View](./src/gui/views/config_name.rs)
* [About View](./src/gui/views/about.rs)
* The **"Current Tracker Information"** and the area where **"Drag and Drop"** and the **progress bar** lie can be found [here](./src/gui/views/trackers.rs). Most of the application's logic take place there (> 500 loc), so I'd advice you have a look.


<!-- ## Structure -->


### Subscription

The application needs to communicate with the subscription to rip stuff

Iced Application <-> Subscription <-> Thread

There's probably a better way to do this.

## Bridging the backend with the frontend
The backend ([xmodits-lib](/src/)) is not asynchronous


# Screenshots
![xmodits gui](/extras/screenshots/Screenshot_1.png) 
![xmodits gui](/extras/screenshots/Screenshot_2.png) 

