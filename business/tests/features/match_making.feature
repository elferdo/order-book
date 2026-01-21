Feature: Compatible asks and bids are matched
    Rule: Highest bid matches lowest ask

    Scenario: One compatible ask and bid
        Given a buyer named Bob
        And   a seller named Susan
        When  Susan sends an ask order not below 2.34
        And   Bob sends a bid order not above 5.0
        Then  buyer Bob has 1 candidate matching seller Susan's ask
        And   seller Susan has 1 candidate matching buyer Bob's bid
