const Asami = artifacts.require("Asami");
const MockDoc = artifacts.require("MockDoc");


module.exports = async function(callback) {
  const accounts = await web3.eth.getAccounts();
  const asami = await Asami.deployed();
  const doc = await MockDoc.deployed();

  const browserAddress = process.env.BROWSER_ADDRESS;
  const adminAddress = process.env.ADMIN_ADDRESS;

  for (let s of topics) { await asami.addTopic(s); }
  await asami.setAdmin(adminAddress, adminAddress);

  for (let a of [adminAddress, browserAddress]) {
    await doc.transfer(a, web3.utils.toWei("50", "ether"), { from: accounts[0] });
    await web3.eth.sendTransaction({from: accounts[0], to: a, value: web3.utils.toWei('100', 'ether')})
  }
  console.log(`{"asami":"${asami.address}", "doc":"${doc.address}"}`);
  callback();
}

const topics = [
  "Fitness",
  "Functional Training",
  "Nutrition and diets",
  "Cooking and Recipes",
  "Food critic",
  "Vegan",
  "Makeup",
  "Hairdressing",
  "Skincare",
  "Low carb",
  "Gluten free",
  "Fashion trends",
  "Gentlemen style",
  "Travel and exploring",
  "Parenting Tips",
  "Relationship advice",
  "Gender exploration",
  "Mental Health",
  "Motivation and Self-Improvement",
  "Home Improvement and DIY",
  "Wine, whisky, alcohol",
  "Psychedelic stuff",
  "Videogames",
  "Retro Gaming",
  "E Sports",
  "Playstation",
  "Nintendo",
  "Xbox",
  "PC Master race",
  "Music releases",
  "Live music",
  "Indie bands",
  "Raving and partying",
  "Photography",
  "Film and Cinema",
  "Blockbusters",
  "Fantasy movies",
  "Cars",
  "Motorbikes",
  "Ciclying",
  "Electric vehicles",
  "Comedy and Humor",
  "Memes and Viral Content",
  "Sports",
  "Futbol",
  "Books and Literature",
  "Theater",
  "Pets and Animals",
  "Gardening",
  "Education ",
  "Primary education",
  "Secondary education",
  "Tertiary education",
  "Art and Creativity",
  "History",
  "Small business",
  "Entrepreneurship",
  "Legal advice",
  "Consumer rights",
  "Finance and Investing",
  "Traditional finance",
  "Decentralized finance",
  "Digital Marketing",
  "Real Estate",
  "Technology Trends",
  "Gadgets",
  "Mobile phones",
  "Computers",
  "Android",
  "iOS",
  "Tech Gadgets",
  "Cryptocurrencies",
  "Bitcoin",
  "Artificial Intelligence",
  "Metaverse",
  "Science",
  "Climate Change",
  "Science and Astronomy",
  "Politics",
  "Health policy",
  "International affairs",
  "Liberalism",
  "Libertarianism",
  "Conservatism",
  "Environment and Sustainability",
  "Social Justice",
];
