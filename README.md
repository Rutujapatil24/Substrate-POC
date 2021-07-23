# Substrate Travels Booking System 
Substrate is a next generation framework for blockchain innovation.The project travels booking system is developed in the substrate for booking travels for the customer. 
## Substrate Node Template
Go to this https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/ for creating substrate node template.
## Substrate Pallet Template
This pallet acts as a template for building other pallets.
Go to this https://github.com/substrate-developer-hub/substrate-pallet-template/blob/master/README.md for creating own pallet in runtime.

## Project Components
There are two pallets info and travels.
The  travels booking system application is comprised of a number of a modules:

#### Info Pallet :

This pallet store the customer details. In customer details contains customer name, customer mobile number,Number of Members,starting location (Source), finish location (Destination), start date, end date, travels type (which type of travels you want i.e sleeper,semi-sleeper), Kilometer, per kilometer cost.Also find total cost with the help of kilometer and perkilometer cost.This pallet is used for travels booking.

#### Travels Pallet:

This pallet store the travels details. In travels details contains travels number, travels name, travels type, number of seats. Whenever we book the travels, we check the travels details, such as which type of travel the customer wants, how many number of seats travels the customer wants, check the that type of travels is available or not. 



## Demo Steps:
1. First Run the following command to start your node:
 ```sh
   ./target/release/node-template --dev --tmp
 ```
2. use this link to navigate to the Polkadot{JS} Apps UI and configure it to connect to the local  chain network: https://polkadot.js.org/apps/#/explorer?rpc=ws://127.0.0.1:9944. 

![Screenshot from 2021-07-22 16-45-26](https://user-images.githubusercontent.com/85154086/126631854-57480c12-821e-4c31-aa9c-e1f47e9705b0.png)

3. Click on Developer. In the developer click on extrinsic. In extrinsic select the Info pallet . After selecting the info pallet. It display customer details form.
 After filling the customer details form do the transaction.

![Screenshot from 2021-07-20 13-05-30](https://user-images.githubusercontent.com/85154086/126481845-e98c1bb6-76c4-45ac-bcc1-dd205373ff68.png)

4. Check the result in the network. In the network go to explorer and see the result of recent events. Also, check the result using the searching block number.

![image 3](https://user-images.githubusercontent.com/85154086/126488849-7db6801a-0212-4fdf-a629-223eec7196bb.png)


5.The same steps will be followed for the travels pallet.
