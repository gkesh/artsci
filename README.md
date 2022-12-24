# artsci
Random CLI ASCII art print utility built in rust. The application can be added to your shell to give out random ascii art when a terminal instance is opened.

### Requirements
Application is compliled using the nightly release of rust, so it is recommended to have that over the stable release. The recommended version of rust is 1.66.0 or over.


### Installation steps
It is a very simple application and super easy to install. In order to get this application running follow the following steps:

1. Clone the repository
```bash
git clone https://github.com/gkesh/artsci.git
```
2. Enter the project folder
```bash
cd artsci
```

3. Run the install script (as *superuser*)
```bash
sudo ./install.sh
```

This should install the application to the system, please test by running the cli app and it should display an in-built ascii art.

```bash
artsci
```

### Configuration
As for configuration, currently you can only add new ascii art and you can do this by creating a new file with extension *ars* inside *~/.config/artsci/ascii*. By defualt you have 6 different ascii arts.

### Screenshots
![screen_1](/artifacts/screens/3_window_shot.png)

> #### Disclaimer
> I claim no ownership to the art used in this app and shown in the screenshots. All of the arts used were pulled from [ASCII Art](https://www.asciiart.eu/). Please visit there, if you wanna add some more designes to your instance.