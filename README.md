# DS210FinalProject

Rukevwe Omusi

5/7/23

Report describing what data set you picked, what interesting thing you discovered, what algorithms you implemented and anything else you consider relevant 

The data set I picked is an Amazon product co-purchasing network collected from the Amazon website based on the site’s Customers Who Bought This Item Also Bought feature. 
The data set was collected by June 1, 2003. It has 403394 nodes and 3387388 edges. If product (node) i is frequently co-purchased with product j, the graph contains a 
directed edge from i to j. I used another file to get the Amazon product metadata that contains product metadata and review information about all the different products 
including books, music CDs, DVDs and VHS video tapes, and the node IDs are the same IDs as those in the product metadata file. 
I implemented the PageRank algorithm to find the PageRank score and a nearest neighbor algorithm to find the connected products to the third degree.
In the metadata file, every product had a list of “similar” products. The nearest neighbor algorithm produced a list of products that was not the same as the list of 
similar products, but the products were similar in and of themselves. For example, product 50 was a music product called “Still Life” and its list of similar products 
had music products like “Alive to Every Smile” and “Alaska”. The neighbors algorithm did not have any music products, but it had multiple educational books such as 
“Science and Poetry” and  “Learning in Overdrive: Designing Curriculum, Instruction, and Assessment from Standards : A Manual for Teachers".
Because of the extremely large data set, the program takes about 12 minutes to run.

The dataset had a small enough file size to be pushed to Github. The data set file is Amazon0601.txt. The metadata file is here: https://snap.stanford.edu/data/amazon-meta.html 

I discovered some interesting connections between items that customers bought such as anime movie DVDs like DBZ being connected with rock music and other connections while sifting through all of the data entries. I also saw a very low tendency to rate anything lower than 4, if ratings were given at all.
