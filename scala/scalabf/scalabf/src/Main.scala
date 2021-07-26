import scala.collection.mutable.ArrayBuffer;
import scala.io.StdIn.{readLine, readChar};

class BfVM(val raw_input: String) {
	private var input = raw_input.toList;

	private var ip: Int = 0;
	private var memory = new ArrayBuffer[Char](); memory += 0;
	private var ptr: Int = 0;
	private var labels = new ArrayBuffer[Int]();

	private def runOne() = {
		ip += 1;
		val current = input(ip - 1);
		val current_val = memory(ptr);
		current match {
			case '>' => {
				if (memory.length <= ptr + 1) memory += 0;
				ptr += 1;
			}
			case '<' => ptr = if (ptr == 0) memory.length - 1 else ptr - 1;
			case '+' => memory(ptr) = if (current_val == 255) 0 
									  else (current_val + 1).toChar;
			case '-' => memory(ptr) = if (current_val == 0) 255 
									  else (current_val - 1).toChar;
			case '[' => labels += ip;
			case ']' => if (memory(ptr) > 0) ip = labels(labels.length - 1) else labels.remove(labels.length - 1);
			case '.' => print(current_val);
			case ',' => memory(ptr) = readChar;
			case _ => {};
		}
	}
	def execute() = while (ip < input.length) {
		this.runOne();
	};
	def reset(in: String) = {
		input = in.toList;
		memory = ArrayBuffer(); memory += 0;
		labels = ArrayBuffer();
		ptr = 0;
		ip = 0;
	}
}

object Main {
	def main(_args: Array[String]): Unit = {
		var vm = new BfVM("""++[->++++<]>+[->++++++++++<]>---.
			<++[->+++++++<]>.
			<++[->+++<]>+.
			<++[->----<]>-.
			<++++[->+++<]>.
			--.
			<++[->----<]>.
			<+++++++[->----------<]>+.
			<++++++++++[->++++++++<]>++++.
			-----.
			<++++++++++[->--------<]>+.
			<++++++++++[->+++++<]>+.
			<++++[->++++<]>.
			--.
			<+++++[->++<]>+.
			<+++++[->--<]>-.
			<++++++++[->----<]>+.
			++++.
			>+++++[->++<]>.""");
		vm.execute();

		while (true) {
			vm.reset("+++[->++++<]>[->+++++<]>++.<++++++[->-----<]>.")
			vm.execute();
			vm.reset(readLine);
			vm.execute();
		}
	}
}
