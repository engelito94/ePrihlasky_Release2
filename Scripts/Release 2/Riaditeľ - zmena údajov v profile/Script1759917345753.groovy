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
import portal.Prihlasovanie as Prihlasovanie
import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

Mail mail = new Mail()

prihlasovanie.prihlasRiaditela('930571647', 'kBvKxcei4AY8p2seZp2QWw==', GlobalVariable.F2A, '910019568')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/span_Menu_inicialy-osoby-header'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/a'))

def email = ''

def hesloMail = ''

if (WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/div_E-mailov adresa_profil-riaditel-mail'), 
    'katalontest987@gmail.com', FailureHandling.OPTIONAL)) {
    email = GlobalVariable.mailSekundarnyLogin

    hesloMail = GlobalVariable.mailSekundarnyHeslo
} else {
    email = GlobalVariable.mailLogin

    hesloMail = GlobalVariable.mailHeslo
}

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/a_Martin Martinsk_font-bold govuk-link'))

def cislo1 = new Random().nextInt(900) + 100

def cislo2 = new Random().nextInt(900) + 100

def telefon = ('+421999' + cislo1.toString()) + cislo2.toString()

'Zmena tel. čísla'
WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_et a zabezpeenie  ePrihlky/input_Zadajte telefnne slo vo formte s pred_01361c'), 
    telefon)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_et a zabezpeenie  ePrihlky/div_-_form-container'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_et a zabezpeenie  ePrihlky/button_Telefnne slo nebolo zadan v sprvnom _fcac9e'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/span_check_circle_panel-text'), 
    'Zmeny sme úspešne uložili.')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_et a zabezpeenie  ePrihlky/a_katalontest987gmail.com_zmenit-email'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_et a zabezpeenie  ePrihlky/input_Na nov emailov adresu Vm poleme overo_0e12bd'))

'Zmena e-mailu'
WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_et a zabezpeenie  ePrihlky/input_Na nov emailov adresu Vm poleme overo_0e12bd'), 
    email)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_et a zabezpeenie  ePrihlky/button_Sp_btnZmenitEmail'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_et a zabezpeenie  ePrihlky/button_Sp_btnZmenitEmail'))

//získanie kódu z emailu
def kod = mail.getSixDigitNumberFromLastEmail('pop.gmail.com', 'pop3', email, hesloMail)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-overenie-input'), 
    kod.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_et a zabezpeenie  ePrihlky/button_Znova odosla kd_Overi'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_et a zabezpeenie  ePrihlky/span_Menu_inicialy-osoby-header'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_et a zabezpeenie  ePrihlky/a'))

WebUI.waitForJQueryLoad(30)

WebUI.delay(5)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/div_Telefnne slo_profil-riaditel-tel'), 
    telefon)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/div_E-mailov adresa_profil-riaditel-mail'), 
    email)

'Odhlásenie'
prihlasovanie.odhlasPouzivatela()

