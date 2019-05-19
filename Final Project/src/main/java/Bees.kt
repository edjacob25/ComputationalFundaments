import BinPacking.Solver.BinPackingSolver
import BinPacking.Solver.Feature
import BinPacking.Solver.Heuristic
import BinPacking.Solver.HyperHeuristic
import BinPacking.Utils.BinPackingProblemSet
import java.util.*
import kotlin.random.Random

class Bees(
        features: Array<out Feature>?,
        heuristics: Array<out Heuristic>?,
        seed: Int,
        private val params: BeesParameters,
        private val trainingSet : String
) : HyperHeuristic(features, heuristics) {

    private val random = Random(seed)
    private var best: Pair<Array<DoubleArray>, Double> = Pair(createRandomBee(), Double.MAX_VALUE)
    init {
        val problems = BinPackingProblemSet(trainingSet)
        var population = (0 until params.numOfBees)
                .map { Pair(createRandomBee(), 0.0) }
                .toMutableList()

        for (gen in 1..params.generations) {
            // Evaluate population on the problem
            population = population.map { Pair(it.first, evaluate(it.first)) }.toMutableList()

            // Sort population
            population.sortBy { it.second }

            // Select the best of exists
            if (population[0].second < best.second) {
                best = population[0]
                println("Best now is ${best.first}")
            }

            val nextGen = mutableListOf<Pair<Array<DoubleArray>, Double>>()

            for ((i, item) in population.take(params.bestBees).withIndex()) {
                val neighSize = if (i < params.eliteBees) params.descendantsOfEliteBees else params.descendantsOfBestBees
                nextGen.addAll(generateNeighbors(item.first, neighSize))
            }
            nextGen.addAll(createScoutBees(params.numOfBees - params.bestBees))
            population = nextGen

            if ( params.patchSize > 0.1 && params.patchDecrementPossibility > random.nextDouble()) {
                params.patchSize = params.patchSize - 0.05
            }
        }
    }

    override fun getHeuristic(solver: BinPackingSolver?): Heuristic {

        val state = features.map { solver!!.getFeature(it) }.toDoubleArray()

        val chosen = best.first
                .mapIndexed { index, array -> Pair(index, calculateDistance(array, state)) }
                .maxBy { it.second }!!.first

        return heuristics[chosen]
    }

    private fun calculateDistance(feature: DoubleArray, actual: DoubleArray ): Double{
        var sum = 0.0
        for (i in features.indices) {
            sum += Math.pow(feature[i] - actual[i], 2.0)
        }
        return Math.sqrt(sum)
    }

    private fun generateNeighbor(original: Array<DoubleArray>): Array<DoubleArray> {
        val neighbor = original.copyOf()
        for (i in 0 until heuristics.size) {
            for (j in 0 until features.size) {
                neighbor[i][j] = neighbor[i][j] + random.nextDouble(-params.patchSize, params.patchSize)
            }
        }
        return neighbor
    }

    private fun generateNeighbors(original: Array<DoubleArray>, neighSize: Int): Collection<Pair<Array<DoubleArray>, Double>> {
        return (0 until neighSize).map { Pair(generateNeighbor(original), 0.0) }
    }

    private fun createScoutBees(number: Int): Collection<Pair<Array<DoubleArray>, Double>> {
        return (0 until number).map { Pair(createRandomBee(), 0.0) }
    }

    private fun evaluate(bee: Array<DoubleArray>): Double {
       return random.nextDouble()
    }

    private fun createRandomBee(): Array<DoubleArray> {
        val result = Array(heuristics.size) { DoubleArray(features.size)}
        for (i in 0 until heuristics.size) {
            for (j in 0 until features.size) {
                result[i][j] = random.nextDouble()
            }
        }
        return result
    }

    override fun toString(): String {
        val string: StringBuilder = StringBuilder()
        for ((i, array) in best.first.withIndex()) {
            string.append(Arrays.toString(array)).append(" => ").append(heuristics[i]).append("\n")
        }
        return string.toString().trim { it <= ' ' }
    }
}

data class BeesParameters(val numOfBees: Int, val generations: Int, val bestBees: Int, val eliteBees: Int,
                          var patchSize: Double, val patchDecrementPossibility: Double,
                          val descendantsOfBestBees: Int, val descendantsOfEliteBees: Int)