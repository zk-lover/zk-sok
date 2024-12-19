#include <stdlib.h>
#include <iostream>

#include "libsnark/zk_proof_systems/ppzksnark/r1cs_ppzksnark/r1cs_ppzksnark.hpp"
#include "libsnark/common/default_types/r1cs_ppzksnark_pp.hpp"
#include "libsnark/gadgetlib1/pb_variable.hpp"
#include "libsnark/gadgetlib1/gadget.hpp"
#include "libsnark/gadgetlib1/gadgets/basic_gadgets.hpp"


using namespace libsnark;

/**
* The LongsightF function can be represented as a circuit:
*
*         L       R
*        x_1     x_0
*         _       _
*         |       |
*         |--------------------.
*         |       |            |
*         v       |            |
* C_0 |->(+)      |            |    j[i] = x[i+1] + C[i]
*         |       |            |
*         v       |            |
*       (^5)      |            |    k[i] = j[i]^5
*         |       v            |
*          `---->(+) = x_2     |  x[i+2] = x[i] + k[i]
*                      _       |
*                      |       |
*                      |--------------------.
*                      |       |            |
*                      v       |            |
*              C_i |->(+)      |            |
*                      |       |            |
*                      v       |            |
*                    (^5)      |            |
*                      |       v            |
*                      `----->(+) = x_(i+2) |
*                                   _       |
*                                   |       |
*                                   v       |
*                       C_(i-1) |->(+)      |
*                                   |       |
*                                   v       |
*                                  (^5)     |
*                                   |       v
*                                   `----->(+) = output
*
*  The round function can be expressed as:
*
*       x[i+2] = x[i] + (x[i+1] + C[i])^5
*
*  Where x[] must start with at least 2 values
*
*  If the values x[0] and x[1] are the variables L and R
*  and x[] is going to be the intermediate state of the function
*  then the first two rounds must substitute those variables, e.g.
*
*       x[0] = R      + (L      + C[i])^5          when i = 0
*       x[1] = L      + (x[i-1] + C[i])^5          when i = 1
*       x[i] = x[i-2] + (x[i-1] + C[i])^5          when i > 1
*
*       output = x[ len(x) - 1 ]
*
*  Knowing the value of x2, x1 and C then x0 can be easily found, while
*  only knowing x0, C and the result finding x1 isn't as trivial.
*
* (%i1) solve([ x[2] = x[0] + (C+x[1])^5 ], [x[2]]);
*
*                 5         4       2  3       3  2      4      5
* (%o1)    [x  = C  + 5 x  C  + 10 x  C  + 10 x  C  + 5 x  C + x  + x ]
*            2           1          1          1         1      1    0
*
*/

template<typename FieldT>
class LongsightF_gadget : public gadget<FieldT>
{
public:
    const std::vector<FieldT> round_constants;
    const pb_variable<FieldT> start_L;
    const pb_variable<FieldT> start_R;

    pb_variable_array<FieldT> round_squares;
    pb_variable_array<FieldT> rounds;

    LongsightF_gadget(
            protoboard<FieldT> &in_pb,
            const std::vector<FieldT> in_constants,
            const pb_variable<FieldT> in_x_L,
            const pb_variable<FieldT> in_x_R,
            const std::string &in_annotation_prefix="",
            const bool do_allocate=true
    ) :
            gadget<FieldT>(in_pb, FMT(in_annotation_prefix, " LongsightF_gadget")),
            round_constants(in_constants),
            start_L(in_x_L),
            start_R(in_x_R),
            round_squares(),
            rounds()
    {
        // Constants may be initialised after constructor
        // Allow allocation to happen separately
        if( do_allocate ) {
            round_squares.allocate(in_pb, round_constants.size() * 2, FMT(in_annotation_prefix, " round_squares"));

            rounds.allocate(in_pb, round_constants.size(), FMT(in_annotation_prefix, " rounds"));
        }
    }

    void allocate()
    {
        round_squares.allocate(this->pb, round_constants.size() * 2, FMT(this->annotation_prefix, " round_squares"));

        rounds.allocate(this->pb, round_constants.size(), FMT(this->annotation_prefix, " rounds"));
    }

    const pb_variable<FieldT>& result() const
    {
        return rounds[ round_constants.size() - 1 ];
    }

    void generate_r1cs_constraints()
    {
        size_t j = 0;

        for( size_t i = 0; i < round_constants.size() - 2; i++ )
        {
            const pb_variable<FieldT>& xL = (
                    i == 0 ? start_L
                           : rounds[i-1]);

            const pb_variable<FieldT>& xR = (
                    i == 0 ? start_R
                           : (i == 1 ? start_L
                                     : rounds[i-2]));

            // -------------------------------------------------
            // Squares

            // (xL+C[i]) * (xL+C[i]) = j[1]
            this->pb.add_r1cs_constraint(
                    r1cs_constraint<FieldT>(
                            round_constants[i] + xL,
                            round_constants[i] + xL,
                            round_squares[j]));

            // j[1] * (xL+C[i]) = j[2]
            this->pb.add_r1cs_constraint(
                    r1cs_constraint<FieldT>(
                            round_squares[j],
                            round_constants[i] + xL,
                            round_squares[j+1]));

/*            // j[2] * (xL+C[i]) = j[3]
            this->pb.add_r1cs_constraint(
                    r1cs_constraint<FieldT>(
                            round_squares[j+1],
                            round_constants[i] + xL,
                            round_squares[j+2]));

            // j[3] * (xL+C[i]) = j[3]
            this->pb.add_r1cs_constraint(
                    r1cs_constraint<FieldT>(
                            round_squares[j+2],
                            round_constants[i] + xL,
                            round_squares[j+3]));*/

            // -------------------------------------------------
            // Intermediate outputs

            // ((j[(1+i)*4 + 3] + xR) * 1) = x[i]
            this->pb.add_r1cs_constraint(
                    r1cs_constraint<FieldT>(
                            1,
                            round_squares[j+1] + xR,
                            rounds[i]));

            // -------------------------------------------------
            // Move to next block of squares
            j += 2;
        }
    }

    void generate_r1cs_witness()
    {
        size_t h = 0;
        for( size_t i = 0; i < round_constants.size(); i++ )
        {
            const FieldT& xR = (
                    i == 0 ? this->pb.val(start_R)
                           : (i == 1 ? this->pb.val(start_L)
                                     : this->pb.val(rounds[i-2])));

            const FieldT& xL = (i == 0 ? this->pb.val(start_L) : this->pb.val(rounds[i-1]));

            // Intermediate squarings
            auto t = xL + round_constants[i];
            this->pb.val(round_squares[h]) = t * t;        // ^2
            this->pb.val(round_squares[h+1]) = this->pb.val(round_squares[h]) * t;    // ^3
            /*this->pb.val(round_squares[h+2]) = this->pb.val(round_squares[h+1]) * t;    // ^4
            this->pb.val(round_squares[h+3]) = this->pb.val(round_squares[h+2]) * t;    // ^5*/

            // Then intermediate X point
            this->pb.val(rounds[i]) = xR + this->pb.val(round_squares[h+1]);

            // Next block of intermediate squarings
            h += 2;
        }
    }
};

template<typename FieldT>
void LongsightF5p3_constants_fill(std::vector<FieldT> &round_constants)
{
    round_constants.resize(5);
    round_constants[0] = FieldT("16141228610716254494246418850894227058386854269090431665976591549148070459029");
    round_constants[1] = FieldT("5243151816343753305078876980603890071959930727088467525831874325200983521963");
    round_constants[2] = FieldT("11443535355782020179109906759898317837986670862629041082203606862552526224884");
    round_constants[3] = FieldT("16540648805601001920805424948549508869776193505507196889296068473215938422144");
    round_constants[4] = FieldT("13262913797752054119281744993321029046637755854445306089831287067330048370211");
}


template<typename FieldT>
const std::vector<FieldT> LongsightF5p3_constants_assign()
{
    std::vector<FieldT> round_constants;

    LongsightF5p3_constants_fill<FieldT>(round_constants);

    return round_constants;
}

template<typename FieldT>
class LongsightF5p3_gadget : public LongsightF_gadget<FieldT>
{
public:
    LongsightF5p3_gadget(
            protoboard<FieldT> &in_pb,
            const pb_variable<FieldT> &in_x_L,
            const pb_variable<FieldT> &in_x_R,
            const std::string &in_annotation_prefix=""
    ) :
            LongsightF_gadget<FieldT>(in_pb, LongsightF5p3_constants_assign<FieldT>(), in_x_L, in_x_R, in_annotation_prefix, false)
    {
        this->allocate();
    }
};

template<typename FieldT>
void LongsightF152p3_constants_fill(std::vector<FieldT> &round_constants)
{
    round_constants.resize(152);
    round_constants[0] = FieldT("7417153685071709436870056242523351150140358124568764639615525440932715960778");
    round_constants[1] = FieldT("12273340427312385197295762796385327795671962575811940024840593759546334831638");
    round_constants[2] = FieldT("12368100453495145398686222132537215736731326260440990626828154738145801268274");
    round_constants[3] = FieldT("9651145733155794490211326053576694361034818606667513183228325583794376899472");
    round_constants[4] = FieldT("20303335619616155978381470742048315877812764167940323516909831488577167301783");
    round_constants[5] = FieldT("17888865178356431883917545983695109758826770784880594328606968855647639258629");
    round_constants[6] = FieldT("8478317580354501373176458292380558679149924902502061143928851622062985998748");
    round_constants[7] = FieldT("21455924899758747089989225480346709914278429649550529318179397847520048040113");
    round_constants[8] = FieldT("16417369350379315367522182891654869157788643412737589741990281968077844865871");
    round_constants[9] = FieldT("682917629062576672829608733775287194360051408076591572059448215532451174467");
    round_constants[10] = FieldT("12665702952792365399099327165160204890362383530457360553333158232311261720960");
    round_constants[11] = FieldT("3534174848896419384928260291097360402585530849237567164316575670351859072726");
    round_constants[12] = FieldT("11386839565127636504885004604778125396015888023999890510109826185644762581064");
    round_constants[13] = FieldT("1376695646556886255839669708890960710789245325248530377671259442117541483907");
    round_constants[14] = FieldT("15127855832621341439717249659608560463537393475674409716284640208948032642700");
    round_constants[15] = FieldT("18763006803560074063046005439647925058939057435678112482302069006974132041579");
    round_constants[16] = FieldT("20321441852601244372607410911049870819721093555961326058229243572778103665283");
    round_constants[17] = FieldT("16090899272519902864744484156761129857296272389405265426000406021640532217216");
    round_constants[18] = FieldT("21844859259875137552772738208511306802649970038290114392793224645779709172435");
    round_constants[19] = FieldT("14858009852787010209240913484548153238423941781255619903407235170096127619600");
    round_constants[20] = FieldT("3895920542421943018597275281244946074647060958213927965386153727618710347038");
    round_constants[21] = FieldT("4876957689800114217585239241810719525039312286542638355755695376819742701770");
    round_constants[22] = FieldT("8966196148580269733071597051623303364078459383088440750477351661943691296141");
    round_constants[23] = FieldT("9992508507936618980262368485554384075304843461994692947052909495952330978227");
    round_constants[24] = FieldT("8213207186873376674370077438124693040838058862120599832024438190183254022600");
    round_constants[25] = FieldT("9492844592174836738004004534381654382718859706616882676923255121468091514222");
    round_constants[26] = FieldT("1392230417149325234053624060264329385329811870545692021640862695990755045791");
    round_constants[27] = FieldT("45849365405090124700721468208274048446838049012227284033194989496291623102");
    round_constants[28] = FieldT("14659132575322212309890049958039718618962597001878062076141859446547516390074");
    round_constants[29] = FieldT("2908541122346760753631035870025481721310654564399371432733658921014027612547");
    round_constants[30] = FieldT("20860468307469641602742499609749664244164019911257166007787532601261365708411");
    round_constants[31] = FieldT("10569756449714722041993500851534866630072110087062915023483726495822415561187");
    round_constants[32] = FieldT("16509280209796076618646389572524052302898993948934291759353816969756518353725");
    round_constants[33] = FieldT("12252230538778914302018943772047831648666753572157477261836385498635258403283");
    round_constants[34] = FieldT("4691680170737213199062427298336160964015731256576292722835070307788129795414");
    round_constants[35] = FieldT("739198973052305210272749213773139899862639322086073314926356051268559766217");
    round_constants[36] = FieldT("6606786823294876799979229266195256254502506728954224324355521767149616100278");
    round_constants[37] = FieldT("19821007813241945557010214650765993047288780193617682807043772926922753134585");
    round_constants[38] = FieldT("1570349884989094651636410079583601855665422535709713017466319979548922798395");
    round_constants[39] = FieldT("10414991091377820567651324652162561256475058358320941810791476912447394534272");
    round_constants[40] = FieldT("2547002771208583964672073330006697377129808841049033501255187735066595441718");
    round_constants[41] = FieldT("7845022062141551911453429755128156185283307470116152832401506005059932067091");
    round_constants[42] = FieldT("17358916670130477544160076977373346440739580397991978466184881065742111893923");
    round_constants[43] = FieldT("17917374666441479266138525808863097826742386025318629339542647877992859962032");
    round_constants[44] = FieldT("10210425665809845167292580495111929375833382009871335167817174069666596287751");
    round_constants[45] = FieldT("4774303698188947042063835014106664756186088744552103981528562359848631485514");
    round_constants[46] = FieldT("15013656347639903130351822120528136849475183817209062094056321428149768853709");
    round_constants[47] = FieldT("6116989140092301217799949835447781205749720800972087611202239105286550290530");
    round_constants[48] = FieldT("7188903758229187317586137610379065486124311000425835028527430325518334131345");
    round_constants[49] = FieldT("1669194105946370318508015637545243105077097546828523569264387618493292069845");
    round_constants[50] = FieldT("19302786148931481892590684019559940395018750005577429780137168918620419910440");
    round_constants[51] = FieldT("9803425938820015666552652915335420200382867307737686198439104730142912427542");
    round_constants[52] = FieldT("9252731062648734326460850251372612101910484571280232427295746483705777927259");
    round_constants[53] = FieldT("11698464170878494841320629727264716514369887689887224178941058825809619097415");
    round_constants[54] = FieldT("5386784685825408467864467159688074900898857988869168087532640901292048458046");
    round_constants[55] = FieldT("9979514450087564608802980748910771714339320977738152911251785421549150652338");
    round_constants[56] = FieldT("20185419783081006932498696550953454662640391337685347107242331763636613686183");
    round_constants[57] = FieldT("21046796270960519753618276383881475004224794458528653294877645932697725859402");
    round_constants[58] = FieldT("8781387611800012433024378544832657792922153766738535323273554036145371414711");
    round_constants[59] = FieldT("19277171738611529769313194029747619929747028152969289540664055203788985755216");
    round_constants[60] = FieldT("10007322809998377168093862186310000486855039772692122928713977184647861549456");
    round_constants[61] = FieldT("19850340785575438062453635882079820839231940029989379089750924430281207936661");
    round_constants[62] = FieldT("8153851869928191594668519108354829795204447427615756565627340658107217394340");
    round_constants[63] = FieldT("2071216420980625731083408257676702219199946500385214060549373884859850016280");
    round_constants[64] = FieldT("8550203374197618805076197699711172830805974981127886360828631845780929086945");
    round_constants[65] = FieldT("1434956537406879832911567747631523283573976586291202860478672602246523412076");
    round_constants[66] = FieldT("18474188318971040498326422102494010954795128959426173313295945784384686970104");
    round_constants[67] = FieldT("12951163141217810990740519670295365179421680761890026493813624671209959831199");
    round_constants[68] = FieldT("4284303668636812405160499403400609483781784208434172474734948072393855164325");
    round_constants[69] = FieldT("12459267201200368874783374465052219445199150634700836384496071869938431836040");
    round_constants[70] = FieldT("4668764912570400877377148712853560163027599706127190842460136854934691184891");
    round_constants[71] = FieldT("5516524269936881203267449218795451990579388055080543439088193855023629493921");
    round_constants[72] = FieldT("11994069982368531651334343359275681617423971622270675023002720444526831152053");
    round_constants[73] = FieldT("2593832637152287975752072197684114609069134156283834385698327029693474220420");
    round_constants[74] = FieldT("2542521037352705685897896714306335678373873126972353913763785410439597475867");
    round_constants[75] = FieldT("16216833311998102291662791292545529519146211962341842320583935942166937956342");
    round_constants[76] = FieldT("5257843447977741403528105168165794690859632408277036851618241990427357320720");
    round_constants[77] = FieldT("11303220060877584558709698626035245028390479088812623136152555812315257834428");
    round_constants[78] = FieldT("3551824333042109470852406240424428172781769929056709738946164861897196750336");
    round_constants[79] = FieldT("21009876812582817444585859584143918798798734480750420702234141462581999190150");
    round_constants[80] = FieldT("7403714332849715309587058878778972200857894940349653824808708048331785222351");
    round_constants[81] = FieldT("10429221612576132125028063044484726056766331274860067857660092944108998204670");
    round_constants[82] = FieldT("10431750210540658163750845749060080474624949954855215145422683385603030980439");
    round_constants[83] = FieldT("20391613990707807649694951405599212485252010662738353181604500934297076162569");
    round_constants[84] = FieldT("7706799434860837579956815829571787323192757806454559455730941246103977314723");
    round_constants[85] = FieldT("11651800880441285525536610182995157660930412370226623648496051478105517487299");
    round_constants[86] = FieldT("9895030577340670358925719304581869365029283156472074930108815271125854800247");
    round_constants[87] = FieldT("5613542832637397167833521947104544050671247288185688658633295166976967783000");
    round_constants[88] = FieldT("13212035985824002768098696126854508734629745681232496430694565813381975622337");
    round_constants[89] = FieldT("15036124018462131955083071688923157949380357499638452313583394878453461905419");
    round_constants[90] = FieldT("12407262221733311455410381202712987045317736452140188040019860701137848889410");
    round_constants[91] = FieldT("20293486529363480955663338971804183115934899086425805344672868778924595835211");
    round_constants[92] = FieldT("3993867844354237910693091659368831635021555916761149555636172224922184428743");
    round_constants[93] = FieldT("19355253291598568429755723505583033331300117279318057230423336623835725314398");
    round_constants[94] = FieldT("7709081952692690648564639919627372849151273570499404639786979548014483980593");
    round_constants[95] = FieldT("4554050911361626913171591846818043031753406488709990580391583551512038281721");
    round_constants[96] = FieldT("5133532761266390878996194891406619586247331561492904944181820895109441306736");
    round_constants[97] = FieldT("5428360169494587891130535937021932553694436933289629685110773645136427470978");
    round_constants[98] = FieldT("13635697621648178330035052081270950847826626926654765863897125170761627616983");
    round_constants[99] = FieldT("7119042655180848118475638831378329823163241579660022976761162813490569186962");
    round_constants[100] = FieldT("6957540751614160604598621115329952574481203167204606201293827592886249357885");
    round_constants[101] = FieldT("17641729056996610893834704880654622515183467729491535180060408710964074879374");
    round_constants[102] = FieldT("5490324605856685861793010524330229446363694673175007443249552808586113192405");
    round_constants[103] = FieldT("21594337847182971778768449052448045414791401414337331671599845580578430463481");
    round_constants[104] = FieldT("17798448368901156082611597692658562318963722301480370763409944668282128885618");
    round_constants[105] = FieldT("2834345661907675914312858220498307840694725827698732696279451138578978828977");
    round_constants[106] = FieldT("241561366142948071664572160006720944009990194585820080390611553415298463744");
    round_constants[107] = FieldT("14516133537943236664623339051197450632015568621015309774627377299273225391014");
    round_constants[108] = FieldT("15762405996596779254336023997068448183910539228518679783726935460553468287001");
    round_constants[109] = FieldT("10568284279172688416298619368642585745991151802088502026309028252542361766216");
    round_constants[110] = FieldT("5734025207066670155969421234069585910343087830069283654430605234599480200673");
    round_constants[111] = FieldT("9096300171828497773859695813024607718032513235287331088579410016257305044372");
    round_constants[112] = FieldT("236165146667143767882839904157663995185896889598560259374558354820998257752");
    round_constants[113] = FieldT("19864668286343389274752059597944120866534881841220750011968366472411381180422");
    round_constants[114] = FieldT("15553205675846950966230740396258968167181880849069792177588431864826132874266");
    round_constants[115] = FieldT("6842522916642154024489607771863497196720833416652714822203065803515647955172");
    round_constants[116] = FieldT("13913592507648117009601455725148951368153520492714080447022152391992303654037");
    round_constants[117] = FieldT("14863290307437539757792191691592764372208806151214018732939039300604493087179");
    round_constants[118] = FieldT("8774157862216267538275739158538621980686797426906144843596232854705570289696");
    round_constants[119] = FieldT("11800452460295708826891458036657613962999270001879476098644239264079261681789");
    round_constants[120] = FieldT("18939335175511962320165921138323748038091399996171438671321023879830074614333");
    round_constants[121] = FieldT("5261917075804436999731693005011674482968798738869149159024566420466968401432");
    round_constants[122] = FieldT("9289684064474145000472875350329796229988550306160124892056113633645696636349");
    round_constants[123] = FieldT("15490464088179712646217802324721323441125431020171778095342949525356002499584");
    round_constants[124] = FieldT("18708128865689742523353197471028955101666440511313963772476259012495703162500");
    round_constants[125] = FieldT("1022172720873280510702275071924020588528705594036898555757640772273365770591");
    round_constants[126] = FieldT("4206044709335147970026691157560711105333353458102464239541203150469787724660");
    round_constants[127] = FieldT("210663558534869764490178804880128176532871332902654566524321788877377700327");
    round_constants[128] = FieldT("5165497450787176266636986278850531744608750268787642340860709279543780465304");
    round_constants[129] = FieldT("16545742665515211269981774899682783145592507921872760606033403745273005831479");
    round_constants[130] = FieldT("5359938212112779702295738602172677787733193743237422852230531029095311033795");
    round_constants[131] = FieldT("13851970127912518553079246414084291170290443926934335075739753476430225566076");
    round_constants[132] = FieldT("6301583785006745458650325360558316908228002555862491299858634772550690525638");
    round_constants[133] = FieldT("4796477300442768605167993495406170519871764375304327404249170869853987615405");
    round_constants[134] = FieldT("12828117805476628337279025271668848408629037112472413704250031463703558170535");
    round_constants[135] = FieldT("9311938688420651610658414998917962721542897660718300561801254712237562309652");
    round_constants[136] = FieldT("12857858850817992974318222960160773636430481847563170506708822323650088457841");
    round_constants[137] = FieldT("12312856411165464853463516497808810953304704919784582845095353771453076811021");
    round_constants[138] = FieldT("16981965884677111575212723064214481686054454401581652332740133323398730119027");
    round_constants[139] = FieldT("7652602467002391800504671921589505850535449917546422819765009328148164612958");
    round_constants[140] = FieldT("14270996507872904452758216195370259853763971982702674932946522601569557802147");
    round_constants[141] = FieldT("20060865871738137102275880189400725361915325354185758765598371544002264457997");
    round_constants[142] = FieldT("6106744544142983761131585714999990121660900678857306909901397569343846191534");
    round_constants[143] = FieldT("8563676195473246462249996777449591192586134229923759112089963752670148429463");
    round_constants[144] = FieldT("611010606004512643711355911931442600100715519058626064301594106348989641094");
    round_constants[145] = FieldT("20707721012915313120356087722820048619763878722734637009860933546065992644060");
    round_constants[146] = FieldT("8796832201147061610760510057159622441836519381725197741663463997379685990940");
    round_constants[147] = FieldT("10000243983727752644950429698095403963058313600546892066937650065452305616322");
    round_constants[148] = FieldT("14373771090952531208417128837232284300520723241730909009676716577905144914758");
    round_constants[149] = FieldT("19418594920323449325449185011513449411864950744159548396683709369792136382456");
    round_constants[150] = FieldT("21270966443617552677367273459164784057931628221880574776474664044046473864531");
    round_constants[151] = FieldT("440721317227119536209338173221659451853756565591751100024804937685462586233");
}


template<typename FieldT>
const std::vector<FieldT> LongsightF152p5_constants_assign( )
{
    std::vector<FieldT> round_constants;

    LongsightF152p3_constants_fill<FieldT>(round_constants);

    return round_constants;
}


template<typename FieldT>
class LongsightF152p3_gadget : public LongsightF_gadget<FieldT>
{
public:
    LongsightF152p3_gadget(
            protoboard<FieldT> &in_pb,
            const pb_variable<FieldT> &in_x_L,
            const pb_variable<FieldT> &in_x_R,
            const std::string &in_annotation_prefix=""
    ) :
            LongsightF_gadget<FieldT>(in_pb, LongsightF152p5_constants_assign<FieldT>(), in_x_L, in_x_R, in_annotation_prefix, false)
    {
        this->allocate();
    }
};






// --------------------------------------------------- ************************************ -------------------------------------------



// In each round, instead of exponentiation like x^3 or x^5, inverse x^-1 is taken
template<typename FieldT>
class LongsightFInv_gadget : public gadget<FieldT>
{
public:
    const std::vector<FieldT> round_constants;
    const pb_variable<FieldT> start_L;
    const pb_variable<FieldT> start_R;

    pb_variable_array<FieldT> round_inverses;
    pb_variable_array<FieldT> rounds;

    LongsightFInv_gadget(
            protoboard<FieldT> &in_pb,
            const std::vector<FieldT> in_constants,
            const pb_variable<FieldT> in_x_L,
            const pb_variable<FieldT> in_x_R,
            const std::string &in_annotation_prefix="",
            const bool do_allocate=true
    ) :
            gadget<FieldT>(in_pb, FMT(in_annotation_prefix, " LongsightFInv_gadget")),
            round_constants(in_constants),
            start_L(in_x_L),
            start_R(in_x_R),
            round_inverses(),
            rounds()
    {
        // Constants may be initialised after constructor
        // Allow allocation to happen separately
        if( do_allocate ) {
            round_inverses.allocate(in_pb, round_constants.size(), FMT(in_annotation_prefix, " round_inverses"));

            rounds.allocate(in_pb, round_constants.size(), FMT(in_annotation_prefix, " rounds"));
        }
    }

    void allocate()
    {
        round_inverses.allocate(this->pb, round_constants.size(), FMT(this->annotation_prefix, " round_inverses"));

        rounds.allocate(this->pb, round_constants.size(), FMT(this->annotation_prefix, " rounds"));
    }

    const pb_variable<FieldT>& result() const
    {
        return rounds[ round_constants.size() - 1 ];
    }

    void generate_r1cs_constraints()
    {
        size_t j = 0;

        /*auto l = this->pb.val(this->start_L);
        auto r = this->pb.val(this->start_R);
        std::cout << "start_L is " << l << std::endl;
        std::cout << "start_R is " << r << std::endl;*/

        for( size_t i = 0; i < round_constants.size() - 2; i++ )
        {
            const pb_variable<FieldT>& xL = (
                    i == 0 ? start_L
                           : rounds[i-1]);

            const pb_variable<FieldT>& xR = (
                    i == 0 ? start_R
                           : (i == 1 ? start_L
                                     : rounds[i-2]));

            // -------------------------------------------------
            // Inverse

            // (xL+C[i]) * (xL+C[i])^-1 = 1
            this->pb.add_r1cs_constraint(
                    r1cs_constraint<FieldT>(
                            round_constants[i] + xL,
                            round_inverses[j],
                            1));

            // -------------------------------------------------
            // Intermediate outputs

            // (((xL+C[i])^-1 + xR) * 1) = x[i]
            this->pb.add_r1cs_constraint(
                    r1cs_constraint<FieldT>(
                            1,
                            round_inverses[j] + xR,
                            rounds[i]));

            // -------------------------------------------------
            // Move to next block
            j++;
        }
    }

    void generate_r1cs_witness()
    {
        size_t h = 0;
        for( size_t i = 0; i < round_constants.size(); i++ )
        {
            const FieldT& xR = (
                    i == 0 ? this->pb.val(start_R)
                           : (i == 1 ? this->pb.val(start_L)
                                     : this->pb.val(rounds[i-2])));

            const FieldT& xL = (i == 0 ? this->pb.val(start_L) : this->pb.val(rounds[i-1]));

            // Inverse
            auto t = xL + round_constants[i];
            this->pb.val(round_inverses[h]) = t.inverse();

            // Then intermediate X point
            this->pb.val(rounds[i]) = xR + this->pb.val(round_inverses[h]);

            // Next block
            h++;
        }
    }
};