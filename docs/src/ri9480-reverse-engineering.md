# Reverse-Engineering the 230V Gaggia Classic 2019

Despite the many modifications that are available for the various models of the Gaggia Classic, there are only a few resources available on the internet regarding the schematics. These include:

* [A diagram for the 120V Gaggia Classic 2019](https://wiki.wholelattelove.com/Gaggia_Classic_Pro/diagrams_and_manuals)
* [A diagram for the 120V Gaggia Classic](https://wiki.wholelattelove.com/Gaggia_Classic/diagrams_and_manuals)
* [This simplified explanation of the schematics](https://comoricoffee.com/en/gaggia-classic-pro-circuit-diagram-en/)

In order to understand the design of the Gaggia Classic better, we will go through a teardown of a 230V (EU) Gaggia Classic 2019, also commonly called the Gaggia Classic Pro (GCP). This model is identified by the model number RI9480, which is stated on the sticker on the bottom of the machine.

![RI9480 Sticker](assets/images/ri9480-teardown-01.jpg)

While there are differences between the different models, the designs share enough similarities for this teardown to hopefully be helpful even when modifying another model.

## Teardown

To begin the disassembly, loosen the two screws on the top of the brewer.

![Brewer as seen from above](assets/images/ri9480-teardown-02.jpg )

This allows us to lift the top cover, including the cup warmer and the funnel to the water tank. Flipping the cover upside down reveals two ground wires (yellow/green) that connect the ground wire from the C14 connector on the back of the machine to the cup warmer (1), and the cup warmer to the chassis (35, see below).

![Bottom of top plate](assets/images/ri9480-teardown-03.jpg)