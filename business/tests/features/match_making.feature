Feature: Compatible asks and bids are matched
    Scenario: Ask with no bid does not match
        Given a seller named Susan
        And   an ask order not below 2.34 by Susan
        When  market runs
        Then  Susan has 0 candidates
