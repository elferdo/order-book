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
