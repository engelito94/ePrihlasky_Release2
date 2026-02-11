import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.model.FailureHandling.OPTIONAL
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

prihlasovanie.prihlasPouzivatela('2ru2nqqn7x@tiffincrane.com', 'w1oXMoeykcdLiib/wAKM5A==', GlobalVariable.F2A)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/span_Menu_inicialy-osoby-header'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a'))

def meno = ''

def priezvisko = ''

if (WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_Mj profil  ePrihlky/span_CK_meno-osoby'), 
    'Ctibor Kazík', FailureHandling.OPTIONAL)) {
    meno = 'Gábor'

    priezvisko = 'Kladivko'
} else {
    meno = 'Ctibor'

    priezvisko = 'Kazík'
}

def krajina = ''

def obec = ''

def ulica = ''

def supisne = ''

def orientacne = ''

def psc = ''

if (WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_Mj profil  ePrihlky/div_Adresa bydliska_profil-adresa'), 
    'Pernek 87, 05478, Pernek, Slovenská republika', FailureHandling.OPTIONAL)) {
    krajina = 'sloven'

    obec = 'trebati'

    ulica = 'zast'

    supisne = '32'

    orientacne = '98'

    psc = '06578'
} else {
    krajina = 'sloven'

    obec = 'pern'

    supisne = '87'

    psc = '05478'
}

def den = ''

def mesiac = ''

def rok = ''

if (WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_Mj profil  ePrihlky/div_Dtum narodenia_profil-dn'), 
    '07.07.1997', FailureHandling.OPTIONAL)) {
    den = '6'

    mesiac = '6'

    rok = '1996'
} else {
    den = '7'

    mesiac = '7'

    rok = '1997'
}

def email = ''

def hesloMail = ''

if (WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_Mj profil  ePrihlky/div_Kontaktn e-mail_profil-mail'), 
    'katalontest987@gmail.com', FailureHandling.OPTIONAL)) {
    email = GlobalVariable.mailSekundarnyLogin

    hesloMail = GlobalVariable.mailSekundarnyHeslo
} else {
    email = GlobalVariable.mailLogin

    hesloMail = GlobalVariable.mailHeslo
}

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_Mj profil  ePrihlky/a_Gbor Kladivko_font-bold govuk-link'))

'zmena osobných údajov'
WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-meno'), 
    meno)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-priezvisko'), 
    priezvisko)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-datumNarodeniaDen'), 
    den)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-datumNarodeniaMesiac'), 
    mesiac)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-datumNarodeniaRok'), 
    rok)

def cislo1 = new Random().nextInt(900) + 100

def cislo2 = new Random().nextInt(900) + 100

def telefon = ('+421999' + cislo1.toString()) + cislo2.toString()

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_Zadajte telefnne slo vo formte s pred_01361c'), 
    telefon)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/button_Zadajte telefnne slo vo formte s pre_22c29e'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/button_Zadajte telefnne slo vo formte s pre_22c29e'))

WebUI.waitForJQueryLoad(60)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/span_check_circle_panel-text'), 
    'Zmeny sme úspešne uložili.')

'zmena e-mailu'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/a_lafhedquimail.site_zmenit-email'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_Na nov emailov adresu Vm poleme overo_0e12bd'), 
    email)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/button_Sp_btnZmenitEmail'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/button_Sp_btnZmenitEmail'))

//získanie kódu z emailu
def kod = mail.getSixDigitNumberFromLastEmail('pop.gmail.com', 'pop3', email, hesloMail)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-overenie-input'), 
    kod)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/button_Znova odosla kd_Overi'))

WebUI.waitForJQueryLoad(60)

//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/span_check_circle_panel-text'), 'Vašu emailovú adresu sme úspešne zmenili.')
//WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/button_Uvete adresu, na ktor prijmate potov_8de598'))
'zmena adresy'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_Pernek 87, 05478, Pernek, Slovensk re_3cb149'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    krajina)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    obec)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

if (ulica.length() > 1) {
    WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_2'), 
        ulica)

    WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))
}

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    supisne)

if (orientacne.length() > 1) {
    WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
        orientacne)
}

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    psc)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/button__ulozit-adresu'))

//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/span_check_circle_panel-text'), 'Zmeny sme úspešne uložili.')
'kontrola zmenených údajov'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/span_Menu_inicialy-osoby-header'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_et a zabezpeenie  ePrihlky/a'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_Mj profil  ePrihlky/div_Telefnne slo_profil-tel'), 
    telefon)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_Mj profil  ePrihlky/div_Dtum narodenia_profil-dn'), 
    ((((('0' + den) + '.') + '0') + mesiac) + '.') + rok)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_Mj profil  ePrihlky/div_Kontaktn e-mail_profil-mail'), 
    email)

if (ulica.length() > 1) {
    WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_Mj profil  ePrihlky/div_Adresa bydliska_profil-adresa'), 
        'Záštepy 32/98, 06578, Trebatice, Slovenská republika')
} else {
    WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilZZ/Page_Mj profil  ePrihlky/div_Adresa bydliska_profil-adresa'), 
        'Pernek 87, 05478, Pernek, Slovenská republika')
}

'Odhlásenie'
prihlasovanie.odhlasPouzivatela()

