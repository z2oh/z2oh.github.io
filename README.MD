## [jaday.io](https://jaday.io)

I use [cobalt](https://cobalt-org.github.io/) as my static site generator. The backbone CSS is based on [skeleton](http://getskeleton.com/), and the rest of it is homegrown or borrowed from other websites.

The `_dev` folder contains tools or asset generators that I have written to generate content for the site.

The actual site is hosted on the [`master`](https://github.com/z2oh/z2oh.github.io/tree/master) branch. When the [`dev`](https://github.com/z2oh/z2oh.github.io/) branch gets updated, [travis-ci](https://travis-ci.org/) builds my site with cobalt and deploys it to the `master` branch. Currently, I'm installing a self-hosted build of cobalt to travis while I wait for [this commit](https://github.com/cobalt-org/cobalt.rs/commit/3f73c81748070e690f8e9a3a90b8600bbcfc0cca) to make it in the next release. That's why my [travis.yml](https://github.com/z2oh/z2oh.github.io/blob/704b3d916723fdd0313436dde05d7a0380678153/.travis.yml) looks a bit strange. If you are looking to build your cobalt site with travis-ci and don't mind the built in syntax highlighting, you should probably follow the instructions [here](http://cobalt-org.github.io/docs/deployment.html).
