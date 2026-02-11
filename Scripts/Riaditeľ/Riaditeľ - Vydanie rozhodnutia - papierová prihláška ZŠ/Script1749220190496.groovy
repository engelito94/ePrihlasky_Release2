import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

Portal portal = new Portal()

//portal.prihlasUcet2FA(GlobalVariable.login, GlobalVariable.heslo)
portal.prihlasUcet('930570810', 'ctqw/dIPXQi2uJsIdYZ0EQ==', GlobalVariable.F2A, true)

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/div_Vetky                                  _330ccb'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/label_V spracovan'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/label_V spracovan'))

WebUI.waitForJQueryLoad(30)

String meno = WebUI.getText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/menoNaPrihlaske'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/span_V spracovan'), 
    'V spracovaní')

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/span_V spracovan'))

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/a_Nvrh na prijatie'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/a_Rozhodnutia'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/a_Rozhodnutia'))

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/FullText'), meno)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/ButtonSearch'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-RdenPodania'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-RdenPodania'))

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/li_Dnes'))

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/div_Vetky                                  _330ccb_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/label_Nvrh na prijatie'))

WebUI.delay(1)

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/span_Akcia_checkmark'))

WebUI.delay(5)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/button_Sprva rozhodnutkeyboard_arrow_down_rounded'))

'Krok 1/3'
WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/a_Vyda rozhodnutia'))

WebUI.delay(5)

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/div_Chystte sa vygenerova 1 rozhodnutie'), 
    'Chystáte sa vygenerovať 1 rozhodnutie:')

'Krok 2/3'
WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

Date date = new Date()

String datum = date.format('ddMM')

int cislo = (Math.random() * 100).round()

String nazovRozhodnutia = (('ATZS' + datum) + '_') + cislo.toString()

WebUI.setText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/input_(nepovinn)_input-cisloRozhodnutia_1d9_6d5197'), 
    nazovRozhodnutia)

WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/textarea_(nepovinn)_textarea-dovod_1d9bd68d_3d6508'), 
    'value', 'Podľa § 47 ods. 1 zákona č. 71/1967 Zb. o správnom konaní (správny poriadok) v znení neskorších predpisov sa od odôvodnenia upúšťa vzhľadom na to, že v predmetnej veci sa účastníkovi/-om konania v plnom rozsahu vyhovelo a boli splnené zákonné aj ostatné podmienky prijatia dieťaťa na základné vzdelávanie.', 
    0, FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/input_(nepovinn)_input-datumPrijatiaOd_1d9b_c73a26'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/input_(nepovinn)_input-datumRozhodnutia'), 
    0)

'Krok 3/3'
WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/button_Prejs na podpis'))

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/span_listu                                 _8f58a1'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/h2_ROZHODNUTIE'), 
    'ROZHODNUTIE')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/div_Zkladn kola, Komenskho 12, Sobrance'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/p_Riadite zkladnej koly Zkladn kola, Komens_e9c4f4'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/p_prijmam na zkladn vzdelvanie v zkladnej k_79b4bc'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/div_Odvodnenie                            P_5aae32'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/div_Pouenie                                _648ee4'), 
    0)

WebUI.verifyElementVisible(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/img_Pouenie_podpis-img'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/div_Rozhodnutie dostan                     _b35e4a'), 
    0)

'Vygeneruj rozhodnutie'
WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/button_Podpsa a vygenerova (1)'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/a_Sp na zoznam rozhodnut'))

WebUI.waitForJQueryLoad(30)

'Čakanie na spracovanie rozhodnutia 30s'
WebUI.delay(60)

WebUI.setText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/FullText'), meno)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/ButtonSearch'))

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-RdenPodania'))

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/li_Dnes'))

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/div_Vetky                                  _330ccb_1'))

WebUI.click(findTestObject('Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/label_Rozhodnutie o prijatí'))

WebUI.delay(1)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/div_AUTO060425'), 
    nazovRozhodnutia)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/div_Rozhodnut o prijat'), 
    'Rozhodnuté o prijatí')

'Odhlásenie'
WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/div_JN'))

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/a_Odhlsi'))

