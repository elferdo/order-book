Feature: Matching compatible asks and bids
    Scenario: Ask with no bid does not match
        Given a seller named Susan
        And   an ask order not below 2.34 by Susan
        When  market runs
        Then  seller Susan has 0 candidates

    Scenario: Bid with no ask does not match
        Given a buyer named Bob
        And   a bid order not above 2.34 by Bob
        When  market runs
        Then  buyer Bob has 0 candidates

    Scenario: One ask and one bid that are not compatible do not match
        Given a buyer named Bob
        And   a seller named Susan
        And   an ask order not below 3.00 by Susan
        And   a bid order not above 2.00 by Bob
        When  market runs
        Then  seller Susan has 0 candidates
        And   buyer Bob has 0 candidates

    Scenario: One ask and one bid that are compatible do match
        Given a buyer named Billy
        And   a seller named Susan
        And   an ask order not below 2.00 by Susan
        And   a bid order not above 3.00 by Billy
        When  market runs
        Then  seller Susan has 1 candidates
        And   buyer Billy has 1 candidates

    Scenario: One ask and two bids that are compatible, only best bid and ask do match
        Given a buyer named Billy
        And   a buyer named Berto
        And   a seller named Susan
        And   an ask order not below 2.00 by Susan
        And   a bid order not above 3.00 by Billy
        And   a bid order not above 2.50 by Berto
        When  market runs
        Then  seller Susan has 1 candidates
        And   buyer Billy has 1 candidates
        And   buyer Berto has 0 candidates

    Scenario: Two asks and one bid that are compatible, only best ask and bid do match
        Given a buyer named Billy
        And   a seller named Sandra
        And   a seller named Susan
        And   an ask order not below 2.00 by Susan
        And   an ask order not below 2.50 by Sandra
        And   a bid order not above 3.00 by Billy
        When  market runs
        Then  seller Susan has 1 candidates
        And   seller Sandra has 0 candidates
        And   buyer Billy has 1 candidates

    Scenario: Three asks and three bids, all compatible
        Given a buyer named Billy
        And   a buyer named Bobby
        And   a buyer named Bruna
        And   a seller named Sandra
        And   a seller named Sergio
        And   a seller named Susan
        And   an ask order not below 1.50 by Sandra
        And   an ask order not below 2.00 by Sergio
        And   an ask order not below 3.50 by Susan
        And   a bid order not above 5.00 by Billy
        And   a bid order not above 5.50 by Bobby
        And   a bid order not above 6.00 by Bruna
        When  market runs
        Then  seller Sandra matches buyer Bruna
        And   seller Sergio matches buyer Bobby
        And   seller Susan matches buyer Billy
