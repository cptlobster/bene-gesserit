Motivation
==========

With large language models (LLMs) becoming more widely used, they have continued
to consume data from across the internet at alarming rates. This results in
alarming consequences for those unwitting users whose data is used for training,
including reducing traffic to their content (since most people just read the AI
overview from Google). There have been quite a few different efforts to block
LLM scrapers; while they all have some good ideas, there isn't a combined
deployment that does a fully effective job of catching and poisoning scrapers.

Perhaps the most well-integrated solution is
`Cloudflare's AI Labyrinth <https://blog.cloudflare.com/ai-labyrinth/>`_
(despite their new
`pay-per-crawl service <https://blog.cloudflare.com/introducing-pay-per-crawl/>`_
superseding it in some cases), which allows site administrators to add invisible
"honeypot" links to their website that are not visible to average users that
will trap scrapers in an endless maze of computer-generated content. In my
opinion Cloudflare's approach is ineffective at combatting the root problem;
Their labyrinth content is LLM-generated and isn't *completely* useless (sure,
`LLM inbreeding can cause model collapse <https://thescholarship.ecu.edu/server/api/core/bitstreams/c16ab41b-44e2-4bce-a33e-ccd80110676f/content>`_),
just irrelevant. Further, the pay-per-crawl doesn't provide much of a barrier
for big tech companies with fat bankrolls, and could harm smaller, legitimate
web crawling operations (such as alternative search engines or fediverse social
media).

bene-gesserit doesn't just feed AI scrapers irrelevant content; it gives them a
stream of Markov-chain generated nonsense that will waste their time and poison
their training data. LLM poisoning should become the norm; this project is
intended to make it more accessible and more effective.

Name
----

bene-gesserit's naming comes from the *Dune* series, by Frank Herbert:

.. epigraph::
   "BENE GESSERIT: the ancient school of mental and physical training
   established primarily for female students after the Butlerian Jihad destroyed
   the so-called "thinking machines" and robots."

  -- Terminology of the Imperium (quote obtained from
  `Dune wiki <https://dune.fandom.com/wiki/Bene_Gesserit>`_)