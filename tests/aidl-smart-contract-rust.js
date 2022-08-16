const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AidlSmartContractRust;
  const baseAccount = anchor.web3.Keypair.generate();

  console.log("📝 First method initialize");

  let tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Total Datasets Count', account.totalDatasets.toString())

  let inputValue =  {
    name: "name",
    dataType: "dataType",
    accuracyScore: "accuracyScore",
    fileType: "fileType",
    size: "fileSize",
    modelsUsed: ["modelList"],
    librariesUsed: ["libraryList"],
  }
  await program.rpc.uploadDatasetDetails(inputValue, {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
    });

     // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Total Datasets Count', account.totalDatasets.toString())

  // Access gif_list on the account!
  console.log('👀 Datasets List', account.datasetsList)
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();