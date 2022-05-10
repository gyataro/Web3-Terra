require('dotenv').config();

module.exports = {
  custom_tester_1: {
    mnemonic:
      process.env.TEST1_SEED_PHRASE,
  },
  custom_tester_2: {
    privateKey: process.env.TEST2_PRIVATE_KEY,
  },
  bombay: {
    mnemonic: process.env.BOMBAY_SEED_PHRASE,
  }
};
