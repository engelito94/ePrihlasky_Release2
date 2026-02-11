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
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.JavascriptExecutor as JavascriptExecutor
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.WebElement as WebElement
import org.openqa.selenium.By as By

Portal portal = new Portal()

def udaje = portal.vyberUdaje('C:\\KatalonProjects\\ePrihlasky\\Data Files\\randomData.txt')

def meno = udaje.meno

def priezvisko = udaje.priezvisko

def pohlavie = udaje.pohlavie //M/Z

WebDriver driver = DriverFactory.getWebDriver()

'Prihlásenie ZZ a vytvorenie prihlášky'
portal.prihlasUcet('930571860', '4Dq6pk30C2ojB+FTFV6r/A==', GlobalVariable.F2A, false)

WebUI.waitForJQueryLoad(30)

if (WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Vytvori nov prihlku'), 
    0, FailureHandling.STOP_ON_FAILURE)) {
    WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Vytvori nov prihlku'))
} else {
    WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Vytvori prihlku'))
}

WebUI.waitForJQueryLoad(30)

if (WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_add                Prida alie diea'), 
    0, FailureHandling.OPTIONAL)) {
    WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_add                Prida alie diea'))
} else {
    WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_add            Prida diea'))
}

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Prida diea  ePrihlky/input_no_maDietaRCRadio'))

WebUI.setText(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Prida diea  ePrihlky/input_(nepovinn)_input-krstneMeno'), 
    meno)

WebUI.setText(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Prida diea  ePrihlky/input_(nepovinn)_input-priezvisko'), 
    priezvisko)

WebUI.setText(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Prida diea  ePrihlky/input_(nepovinn)_input-datumNarodenia'), 
    '5.5.2022')

if (pohlavie.equals('M')) {
    WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Prida diea  ePrihlky/input_(nepovinn)_pohlavieRadio'))
} else {
    WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Prida diea  ePrihlky/input_mu_pohlavieRadio'))
}

WebUI.click(findTestObject('Zak_test/Modal_Button_Dalej'))

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Prida diea  ePrihlky/input_(nepovinn)_input-miestoNarodenia'), 
    'Slovensko')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/PotvrditNastup/adresaDieta/Page_Prida diea  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/PotvrditNastup/adresaDieta/Page_Prida diea  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'Sloven')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/PotvrditNastup/adresaDieta/Page_Prida diea  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'koru')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/PotvrditNastup/adresaDieta/Page_Prida diea  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2'), 
    'záto')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/PotvrditNastup/adresaDieta/Page_Prida diea  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/PotvrditNastup/adresaDieta/Page_Prida diea  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
    '55')

WebUI.setText(findTestObject('Object Repository/Zak_test/PotvrditNastup/adresaDieta/Page_Prida diea  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    '22')

WebUI.setText(findTestObject('Object Repository/Zak_test/PotvrditNastup/adresaDieta/Page_Prida diea  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    '05148')

WebUI.click(findTestObject('Zak_test/Modal_Button_PridatDieta'))

WebUI.waitForJQueryLoad(60)

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

'MS'
WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Na zkladn kolu_radioGroup-typ-skoly'))

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.waitForElementVisible(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/Button_vymazatFiltre'),
	0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/Button_vymazatFiltre'))

WebUI.delay(2)
WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/Input_AdresaSkoly'), "hôrka")

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/autocomplete_adresa'))

//karta oblubene skoly
//WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/OblubeneSkoly'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Prida do prihlky_1'))

WebUI.waitForJQueryLoad(30)

/*if (WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/PotvrditNastup/skolaHorka'), 0, FailureHandling.OPTIONAL)) {
    WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/skolaHorka'))
} else {
    WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Prida do prihlky'))
}*/

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__zastupca2Radio'))

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_celodenn vchovu a vzdelvanie'))

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_DPDSVVPRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Potvrdenie o zdravotnej spsobilosti'), 
    '4', true)

WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_cloud_upload                           _f133fd'), 
    'C:\\Users\\barcik\\Downloads\\Dokument.pdf')

WebUI.waitForJQueryLoad(30)

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Poznmka_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_(nepovinn)_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Odosla prihlku'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/h1_Prihlka bola spene odoslan'), 
    'Prihláška bola úspešne odoslaná!')

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Prejs na moje prihlky'))

'Koniec vytvárania prihlášky'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Prihlky a rozhodnutia  ePrihlky/span_KL'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Prihlky a rozhodnutia  ePrihlky/a_Odhlsi'))

/*
 * 
 * Začína tu čast pre riaditeľa - vydať rozhodnutie
 * 
 * 
 * */
'Vydanie rozhodnutia'
portal.prihlasUcet('930571647', 'kBvKxcei4AY8p2seZp2QWw==', GlobalVariable.F2A, true)

String menoCele = (meno + ' ') + priezvisko

WebUI.waitForJQueryLoad(30)

WebUI.delay(2)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/PFullText'), 0)

WebUI.setText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/PFullText'), menoCele)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/PButtonSearch'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/StavPodana'))

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/StavSpracovani'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/span_V spracovan'))

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/a_Nvrh na prijatie'))

WebUI.waitForJQueryLoad(30)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/a_Rozhodnutia'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/a_Rozhodnutia'))

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/FullText'), menoCele)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/ButtonSearch'))

WebUI.delay(5)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-RdenPodania'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/div_Vetky                                  _330ccb_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/label_Nvrh na prijatie'))

WebUI.delay(1)

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/span_Akcia_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/button_Sprva rozhodnutkeyboard_arrow_down_rounded'))

'Krok 1/3'
WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/a_Vyda rozhodnutia'))

WebUI.waitForJQueryLoad(30)

WebUI.delay(10)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/div_Chystte sa vygenerova 1 rozhodnutie'), 
    'Chystáte sa vygenerovať 1 rozhodnutie:')

'Krok 2/3'
WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

Date date = new Date()

String datum = date.format('ddMM')

int cislo = (Math.random() * 100).round()

String nazovRozhodnutia = (('ATMS' + datum) + '_') + cislo.toString()

WebUI.setText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/input_(nepovinn)_input-cisloRozhodnutia_1d9_6d5197'), 
    nazovRozhodnutia)

WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/textarea_(nepovinn)_textarea-dovod_1d9bd68d_3d6508'), 
    'value', 'Podľa § 47 ods. 1 zákona č. 71/1967 Zb. o správnom konaní (správny poriadok) v znení neskorších predpisov sa od odôvodnenia upúšťa vzhľadom na to, že v predmetnej veci sa zákonnému zástupcovi/zákonným zástupcom dieťaťa v plnom rozsahu vyhovelo a boli splnené zákonné aj ostatné podmienky prijatia dieťaťa na predprimárne vzdelávanie.', 
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

//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/h2_ROZHODNUTIE'), 'ROZHODNUTIE', FailureHandling.OPTIONAL)
WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/div_Zkladn kola, Komenskho 12, Sobrance'), 
    0, FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/p_Riadite zkladnej koly Zkladn kola, Komens_e9c4f4'), 
    0, FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/p_prijmam na zkladn vzdelvanie v zkladnej k_79b4bc'), 
    0, FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/div_Odvodnenie                            P_5aae32'), 
    0, FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/div_Pouenie                                _648ee4'), 
    0, FailureHandling.STOP_ON_FAILURE)

//WebUI.verifyElementVisible(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/img_Pouenie_podpis-img'), FailureHandling.OPTIONAL)
WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/div_Rozhodnutie dostan                     _b35e4a'), 
    0, FailureHandling.STOP_ON_FAILURE)

'Vygeneruj rozhodnutie'
WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/button_Podpsa a vygenerova (1)'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Vyda rozhodnutia  ePrihlky/a_Sp na zoznam rozhodnut'))

WebUI.waitForJQueryLoad(30)

'Čakanie na spracovanie rozhodnutia 60s'
WebUI.delay(60)

WebUI.setText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/FullText'), menoCele)

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/ButtonSearch'))

WebUI.delay(1)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/div_AUTO060425'), 
    nazovRozhodnutia)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/div_Rozhodnut o prijat'), 
    'Rozhodnuté o prijatí')

'Odhlásenie - koniec vydania rozhodnutia'
WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/div_JN'))

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/Page_Prihlky a rozhodnutia  ePrihlky/a_Odhlsi'))

/*
 * 
 * 
 * Časť ZZ - potvrdenie nástupu
 * 
 * 
 * */
'Začiatok potvrdenia nástupu'
portal.prihlasUcet('930571860', '4Dq6pk30C2ojB+FTFV6r/A==', GlobalVariable.F2A, false)

WebUI.waitForJQueryLoad(30)

WebElement element = null

String elementText = ''

String xpath = ''

boolean najdenaPrihlaska = false

int poradiePrihlasky = 5

while (!(najdenaPrihlaska)) {
    xpath = (('//div[' + poradiePrihlasky.toString()) + ']/div/div/div/div[2]/span')

    try {
        elementText = driver.findElement(By.xpath(xpath)).getText()
    }
    catch (NoSuchElementException e) {
        e.message()
    } 
    
    if (elementText.equals(menoCele)) {
        najdenaPrihlaska = true

        xpath = (('//div[' + poradiePrihlasky.toString()) + ']/div[1]/div[2]/div[2]/a[1]')

       element = driver.findElement(By.xpath(xpath));
	   ((JavascriptExecutor)driver).executeScript("arguments[0].scrollIntoView(true);", element);
	   
	   //element.click();
		
		//WebUI.executeJavaScript('arguments[0].scrollIntoView(true);', element)
	   
		//klik cez javascript - element mimo viewport
        ((JavascriptExecutor)driver).executeScript("arguments[0].click();", element);
    }
    
    poradiePrihlasky++
}

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Rozhodnut o prijat'), 
    'Rozhodnuté o prijatí')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_Potvrdi nstup'), 
    0)

'Potvrdiť nástup'
WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_Potvrdi nstup'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Potvrte prijatie  ePrihlky/div_Potvrdi nstup na kolu mete len pre jedn_eb448c'), 
    'Potvrdiť nástup na školu môžete len pre jednu školu. Pre všetky ostatné školy, kde bolo vydané rozhodnutie o prijatí, bude automaticky označené, že dieťa odmietlo nástup do školy.')

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Potvrte prijatie  ePrihlky/button_Potvrdi nstup'))

//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Potvrte prijatie  ePrihlky/h1_Gratulujeme'), 'Gratulujeme!')
WebUI.waitForJQueryLoad(30)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Potvrte prijatie  ePrihlky/button_Prejs do prihlok'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Potvrte prijatie  ePrihlky/button_Prejs do prihlok'))

WebUI.waitForJQueryLoad(30)

//doplnit otvorenie detailu prihlasky podla logiky vyssie
 ((JavascriptExecutor)driver).executeScript("arguments[0].click();", element);

WebUI.waitForJQueryLoad(30)

WebUI.verifyTextPresent('Rozhodnuté o prijatí', false)

WebUI.verifyTextPresent('nástup potvrdený', false)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_Potvrdi nstup'), 
    0, FailureHandling.OPTIONAL)

WebUI.verifyElementNotVisible(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_Potvrdi nstup'), 
    FailureHandling.OPTIONAL)

'Odhlásenie - koniec potvrdenia nástupu'
WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_M'))

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_Odhlsi'))

