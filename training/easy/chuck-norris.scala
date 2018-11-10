import scala.io.StdIn.readLine
import scala.language.implicitConversions

object Solution extends App {
  class CNStringBuilder(val sb: StringBuilder) {
    def addByte(b: Boolean): StringBuilder = {
      sb.append(if (b) { "0 " } else { "00 " })
    }

    def addCount(count: Int): StringBuilder = {
      sb.appendAll(Array.fill(count)('0')).append(' ')
    }
  }
  implicit def sbToCNSb(sb: StringBuilder): CNStringBuilder = new CNStringBuilder(sb)

  def bytesToBits(b: Byte): IndexedSeq[Boolean] = {
    (6 to 0 by -1).map(i => (b & (1 << i)) > 0)
  }

  def convert(state: (StringBuilder, Boolean, Int), current: Boolean): (StringBuilder, Boolean, Int) = {
    if (state._2 == current && state._3 > 0) {
      (state._1, state._2, state._3 + 1)
    } else {
      (state._1.addCount(state._3).addByte(current), current, 1)
    }
  }

  val res = readLine
    .getBytes("ASCII")
    .flatMap(bytesToBits)
    .foldLeft((new StringBuilder, true, 0))(convert)

  println(res._1.addCount(res._3).toString.trim)
}
