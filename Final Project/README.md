# Hyperheuristic for solving 1D Packing Problem

## Building
Due to our decision to use [Kotlin](https://kotlinlang.org/), a JVM-based language, for the programming of our
hyperheuristic, the build process is a little different than the rest of our classmates. For us to create reproducible
builds and a better, IDE agnostic experience, we decided to use [Maven](https://maven.apache.org/), which is a command
line tool used for automatization of compiling and creating jars in the java world.

Once maven is installed, it its as simple as using:
```bash
$ mvn compile
```
In the carpet with the `pom.xml` file to compile all the project sources and downloads the necessary dependencies from
the web.

To generate the executable jar, it is as simple as using:
```bash
$ mvn install
```
The jar file will be created in the `target` directory.

However, Maven is very integrated with the majority of the java IDE's, which allows us to use the GUI to call on Maven
for us.
## Running
In order to run our project, it is necessary to have, in the same directory as the jar, a directory called `Instances`
with two subdirectories `Training` and `Testing`, each one containing the `.bpp` files which contain the problems to
solve, each one adhering to the specifications set in the last part of the project.

After the directories are ready, it is only needed to issue the following command:
```bash
$ java -jar nameOfTheGeneratedJar.jar
```
Which will generate all the test cases we used for our project in csv format.

If we want to do a rapid test run without compiling and movin the jar to the appropriate place, we can use this Maven
command to run the application:
```
$ mvn org.codehaus.mojo:exec-maven-plugin:1.5.0:java -Dexec.mainClass="Run"
```