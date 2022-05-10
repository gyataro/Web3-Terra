module.exports = ({ wallets, refs, config, client }) => ({
  // This is what we're adding. "clicker" is the contract we want to interact with, "getFortune" is the function.
  getFortune: () => client.query("clicker", { get_fortune: {} }),
  getScores: () => client.query("clicker", { get_scores: {} }),

  upsertScore: (score, signer = wallets.validator) =>
    client.execute(signer, "clicker", { upsert_score: { score } }),
});