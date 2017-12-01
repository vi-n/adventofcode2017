fn parse_input(input: &str) -> Vec<u32> {
    let mut nums: Vec<u32> = vec![];

    for c in input.chars() {
        nums.push(c.to_digit(10).unwrap());
    }

    return nums;
}

fn solve_first_captcha(input: Vec<u32>) -> u32 {
    let mut total: u32 = 0;
    let mut prev: &u32 = input.last().unwrap();

    for num in &input {
        if num == prev {
            total += *num;
        }

        prev = num;
    }

    return total;
}

fn solve_second_captcha(input: Vec<u32>) -> u32 {
    let jump = input.len()/2;
    let mut total: u32 = 0;

    for i in 0..input.len() {
        if input[i] == input[(i + jump) % input.len()] {
            total += input[i];
        }
    }

    return total;
}

pub fn solve_part1(input: &str) -> u32 {
    let nums = parse_input(input);
    return solve_first_captcha(nums);
}

pub fn solve_part2(input: &str) -> u32 {
    let nums = parse_input(input);
    return solve_second_captcha(nums);
}

#[test]
fn test_examples_part1() {
    let inputs      = ["1122",  "1111",     "1234",     "91212129"];
    let solutions   = [3,       4,          0,          9]; 

    for i in 0..inputs.len() {
        assert!(solve_part1(inputs[i]) == solutions[i]);
    }
}

#[test]
fn test_examples_part2() {
    let inputs      = ["1212",  "1221",     "123425",   "123123",   "12131415"];
    let solutions   = [6,       0,          4,          12,         4];

    for i in 0..inputs.len() {
        assert!(solve_part2(inputs[i]) == solutions[i]);
    }
}

#[test]
fn test_given_input() {
    let input = "36743676522426214741687639282183216978128565594112364817283598621384839756628424146779311928318383597235968644687665159591573413233616717112157752469191845757712928347624726438516211153946892241449523148419426259291788938621886334734497823163281389389853675932246734153563861233894952657625868415432316155487242813798425779743561987563734944962846865263722712768674838244444385768568489842989878163655771847362656153372265945464128668412439248966939398765446171855144544285463517258749813731314365947372548811434646381595273172982466142248474238762554858654679415418693478512641864168398722199638775667744977941183772494538685398862344164521446115925528534491788728448668455349588972443295391385389551783289417349823383324748411689198219329996666752251815562522759374542652969147696419669914534586732436912798519697722586795746371697338416716842214313393228587413399534716394984183943123375517819622837972796431166264646432893478557659387795573234889141897313158457637142238315327877493994933514112645586351127139429281675912366669475931711974332271368287413985682374943195886455927839573986464555141679291998645936683639162588375974549467767623463935561847869527383395278248952314792112113126231246742753119748113828843917812547224498319849947517745625844819175973986843636628414965664466582172419197227695368492433353199233558872319529626825788288176275546566474824257336863977574347328469153319428883748696399544974133392589823343773897313173336568883385364166336362398636684459886283964242249228938383219255513996468586953519638111599935229115228837559242752925943653623682985576323929415445443378189472782454958232341986626791182861644112974418239286486722654442144851173538756859647218768134572858331849543266169672745221391659363674921469481143686952478771714585793322926824623482923579986434741714167134346384551362664177865452895348948953472328966995731169672573555621939584872187999325322327893336736611929752613241935211664248961527687778371971259654541239471766714469122213793348414477789271187324629397292446879752673";

    assert!(solve_part1(input) == 1069);
    assert!(solve_part2(input) == 1268);
}
