import BinPacking.Solver.BinPackingSolver
import BinPacking.Solver.Feature
import BinPacking.Solver.Heuristic
import BinPacking.Solver.HyperHeuristic
import BinPacking.Utils.BinPackingProblemSet
import BinPacking.Utils.Files
import java.text.DecimalFormat

/**
 * Runs the bin packing framework.
 *
 *
 *
 * @author José Carlos Ortiz Bayliss (jcobayliss@tec.mx)
 * @version 1.0
 */
object Run {

    @JvmStatic
    fun main(args: Array<String>) {
        val features = arrayOf(Feature.AVGL, Feature.STDL, Feature.SMALL, Feature.VSMALL, Feature.LARGE,
                Feature.VLARGE, Feature.COLORC, Feature.OBINS, Feature.AVGW, Feature.COLORF)

        val heuristics = arrayOf(Heuristic.FIRST_FIT, Heuristic.FIRST_FIT_SC, Heuristic.FIRST_FIT_DC,
                Heuristic.BEST_FIT, Heuristic.BEST_FIT_SC, Heuristic.BEST_FIT_DC, Heuristic.WORST_FIT,
                Heuristic.WORST_FIT_SC, Heuristic.WORST_FIT_DC, Heuristic.ALMOST_WORST_FIT,
                Heuristic.ALMOST_WORST_FIT_SC, Heuristic.ALMOST_WORST_FIT_DC)

        /*
         * Calculates the initial states of a set of problem instances by using some specific features.
         */
        characterizeSet("Instances/Training", "features.csv", features)

        /*
         * Solves a set of problem instances by using some specific heuristics.
         */
        solveSet("Instances/Training", "heuristics.csv", heuristics)

        val parametersList = listOf(BeesParameters(20, 10, 3, 1, 0.5, 0.1, 2, 4),
                BeesParameters(20, 10, 4, 2, 0.5, 0.1, 2, 5),
                BeesParameters(35, 20, 5, 2, 0.5, 0.1, 2, 5),
                BeesParameters(45, 50, 5, 2, 0.5, 0.1, 2, 5),
                BeesParameters(50, 100, 8, 2, 0.5, 0.1, 2, 7),
                BeesParameters(100, 200, 10, 2, 0.5, 0.1, 3, 10),
                BeesParameters(100, 500, 10, 3, 0.5, 0.1, 3, 10),
                BeesParameters(100, 1000, 10, 3, 0.5, 0.1, 3, 10),
                BeesParameters(100, 2000, 10, 3, 0.5, 0.1, 3, 10),
                BeesParameters(100, 5000, 10, 3, 0.5, 0.1, 3, 10))

        for ((i, parameters) in parametersList.withIndex()) {
            val hyperHeuristic = Bees(features, heuristics, i, parameters, "Instances/Training")
            println(hyperHeuristic)
            /*
             * Solves a set of problem instances (the ones used for training) by using the previously defined hyper-heuristic.
             */
            solveSet("Instances/Training", "hyperHeuristic-Training_$parameters.csv", hyperHeuristic)
            /*
             * Solves a set of problem instances (the ones used for testing) by using the previously defined hyper-heuristic.
             */
            solveSet("Instances/Testing", "hyperHeuristic-Testing_$parameters.csv", hyperHeuristic)
        }

        for ((i, parameters) in parametersList.withIndex()) {
            parameters.numOfBees= parameters.numOfBees / 2
            val hyperHeuristic = Bees(features, heuristics, i, parameters, "Instances/Training")
            println(hyperHeuristic)
            /*
             * Solves a set of problem instances (the ones used for training) by using the previously defined hyper-heuristic.
             */
            solveSet("Instances/Training", "hyperHeuristic-Training_$parameters.csv", hyperHeuristic)
            /*
             * Solves a set of problem instances (the ones used for testing) by using the previously defined hyper-heuristic.
             */
            solveSet("Instances/Testing", "hyperHeuristic-Testing_$parameters.csv", hyperHeuristic)
        }

        for ((i, parameters) in parametersList.withIndex()) {
            parameters.numOfBees= parameters.numOfBees * 2
            val hyperHeuristic = Bees(features, heuristics, i, parameters, "Instances/Training")
            println(hyperHeuristic)
            /*
             * Solves a set of problem instances (the ones used for training) by using the previously defined hyper-heuristic.
             */
            solveSet("Instances/Training", "hyperHeuristic-Training_$parameters.csv", hyperHeuristic)
            /*
             * Solves a set of problem instances (the ones used for testing) by using the previously defined hyper-heuristic.
             */
            solveSet("Instances/Testing", "hyperHeuristic-Testing_$parameters.csv", hyperHeuristic)
        }
    }

    /**
     * Characterizes a set of bin packing problem instances by using a set of
     * features and saves the results in a file whose file is provided as argument.
     *
     *
     *
     * @param folder   The folder where the instances are stored.
     * @param fileName The name of the file where the results will be saved.
     * @param features The features to be used to characterize the problem
     * instances.
     */
    private fun characterizeSet(folder: String, fileName: String, features: Array<Feature>) {
        val string: StringBuilder = StringBuilder()
        val format = DecimalFormat("0.0000")
        val set = BinPackingProblemSet(folder)
        var solver: BinPackingSolver
        /*
         * Prints the header of the file.
         */
        string.append("File, ")
        for (feature in features) {
            string.append(feature).append(", ")
        }
        string.delete(string.length - 2, string.length)
        string.append("\n")
        /*
         * Prints the features for each instance in the set.
         */
        for (problem in set.instances) {
            string.append(problem.fileName).append(", ")
            for (feature in features) {
                solver = BinPackingSolver(problem)
                string.append(format.format(solver.getFeature(feature))).append(", ")
            }
            string.delete(string.length - 2, string.length)
            string.append("\n")
        }
        Files.save(string.toString().trim { it <= ' ' }, fileName)
    }

    /**
     * Solves a set of bin packing problem instances by using a set of
     * heuristics and saves the results in a file whose file is provided as argument.
     *
     *
     *
     * @param folder     The folder where the instances are stored.
     * @param fileName   The name of the file where the results will be saved.
     * @param heuristics The heuristics to be used to solve the problem
     * instances.
     */
    private fun solveSet(folder: String, fileName: String, heuristics: Array<Heuristic>) {
        val string: StringBuilder = StringBuilder()
        val format = DecimalFormat("0.0000")
        val set = BinPackingProblemSet(folder)
        var solver: BinPackingSolver
        /*
         * Prints the header of the file.
         */
        string.append("File, ")
        for (heuristic in heuristics) {
            string.append(heuristic).append(", ")
        }
        string.delete(string.length - 2, string.length)
        string.append("\n")
        /*
         * Prints the features and results for each instance in the set.
         */
        for (problem in set.instances) {
            string.append(problem.fileName).append(", ")
            for (heuristic in heuristics) {
                solver = BinPackingSolver(problem)
                solver.solve(heuristic)
                string.append(format.format(solver.getFeature(Feature.AVGW))).append(", ")
            }
            string.delete(string.length - 2, string.length)
            string.append("\n")
        }
        Files.save(string.toString().trim { it <= ' ' }, fileName)
    }

    /**
     * Solves a set of bin packing problem instances by using a set of
     * heuristics and saves the results in a file whose file is provided as argument.
     *
     *
     *
     * @param folder         The folder where the instances are stored.
     * @param fileName       The name of the file where the results will be saved.
     * @param hyperHeuristic The hyper-heuristic to be used to solve the problem
     * instances.
     */
    private fun solveSet(folder: String, fileName: String, hyperHeuristic: HyperHeuristic) {
        val string = StringBuilder()
        val format = DecimalFormat("0.0000")
        val set = BinPackingProblemSet(folder)
        var solver: BinPackingSolver
        /*
         * Prints the header of the file.
         */
        string.append("File, Hyper-heuristic\n")
        /*
         * Prints the features and results for each instance in the set.
         */
        for (problem in set.instances) {
            string.append(problem.fileName).append(", ")
            solver = BinPackingSolver(problem)
            solver.solve(hyperHeuristic)
            string.append(format.format(solver.getFeature(Feature.AVGW))).append("\n")
        }
        Files.save(string.toString().trim { it <= ' ' }, fileName)
    }

}
