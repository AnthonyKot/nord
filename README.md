# nord
Nord trading and lending system template



    src/: Contains the main source code files.
        main.rs: The entry point of the application.
        trading/: Module for handling trading-related functionality.
            mod.rs: Declares the trading module and its public items.
            engine.rs: Implements the trading engine.
            order.rs: Defines the Order struct and related functions.
            matching.rs: Implements the order matching logic.
        lending/: Module for handling lending-related functionality.
            mod.rs: Declares the lending module and its public items.
            engine.rs: Implements the lending engine.
            loan.rs: Defines the Loan struct and related functions.
            interest.rs: Implements interest rate calculation logic.
        margining/: Module for handling margining and risk management.
            mod.rs: Declares the margining module and its public items.
            system.rs: Implements the margining system.
            account.rs: Defines the Account struct and related functions.
            risk.rs: Implements risk management logic.
        utils/: Module for utility functions and shared functionality.
            mod.rs: Declares the utils module and its public items.
            currency.rs: Defines currency-related functions and constants.
            pricing.rs: Implements pricing-related functions.
        db/: Module for database and storage-related functionality.
            mod.rs: Declares the db module and its public items.
            storage.rs: Implements storage and database operations.

    tests/: Contains integration and unit tests for the project.
        trading_tests.rs: Contains tests related to the trading module.
        lending_tests.rs: Contains tests related to the lending module.
        margining_tests.rs: Contains tests related to the margining module.




[dependencies]: Specifies the project's dependencies.

    chrono: A library for date and time handling.
    rust_decimal: A library for decimal arithmetic.
    serde and serde_json: Libraries for serialization and deserialization.
    tokio: An asynchronous runtime for Rust.
    uuid: A library for generating and handling UUIDs.

