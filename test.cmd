#!/bin/bash

MONOLITH_HOME=`pwd`
export PATH=$PATH:$MONOLITH_HOME/target/debug #:$MONOLITH_HOME/target/release


echo "writing A0"
A0=`cli-monolith-db dbtest create "have a nice day"`
echo "reading $A0"
cli-monolith-db dbtest read $A0

echo "writing A1"
A1=`cli-monolith-db dbtest create "have a pleasant evening"`
echo "reading $A1"
cli-monolith-db dbtest read $A1

echo "writing A2"
A2=`cli-monolith-db dbtest create "have a delicious teatime"`
echo "reading $A2"
cli-monolith-db dbtest read $A2

echo "writing A3"
A3=`cli-monolith-db dbtest create "have a delicious cake"`
echo "reading $A3"
cli-monolith-db dbtest read $A3

echo "writing A4"
A4=`cli-monolith-db dbtest create "nothing"`
echo "reading $A4"
cli-monolith-db dbtest read $A4

echo "writing A5"
A5=`cli-monolith-db dbtest create "1;2;3;4;asdf;wert;xvcbxcvb"`
echo "reading $A5"
cli-monolith-db dbtest read $A5

echo "writing A6"
A6=`cli-monolith-db dbtest create "have a delicious teatime"`
echo "reading $A6"
cli-monolith-db dbtest read $A6

echo "writing A7"
A7=`cli-monolith-db dbtest create "have some soup"`
echo "reading $A7"
cli-monolith-db dbtest read $A7

echo "writing A8"
A8=`cli-monolith-db dbtest create "have some veggies"`
echo "reading $A8"
cli-monolith-db dbtest read $A8

echo "writing A9"
A9=`cli-monolith-db dbtest create "the command is wicked!"`
echo "reading $A9"
cli-monolith-db dbtest read $A9

echo "writing A10"
A10=`cli-monolith-db dbtest create "this solution is crooked!"`
echo "reading $A10"
cli-monolith-db dbtest read $A10


cli-monolith-db dbtest read 1675416517992552265
cli-monolith-db dbtest read 1675416517994640166
cli-monolith-db dbtest read 1675416517997904849
cli-monolith-db dbtest read 1675416518000036257
cli-monolith-db dbtest read 1675416518002329344
cli-monolith-db dbtest read 1675416518004426419
cli-monolith-db dbtest read 1675416518007667515
cli-monolith-db dbtest read 1675416518011222566
cli-monolith-db dbtest read 1675416518013573875
cli-monolith-db dbtest read 1675416518015657264
cli-monolith-db dbtest read 1675416518018040527