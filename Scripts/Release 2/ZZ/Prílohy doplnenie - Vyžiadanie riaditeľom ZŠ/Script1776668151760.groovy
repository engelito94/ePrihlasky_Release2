import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.model.FailureHandling.OPTIONAL
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.BrokenTestObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable
import portal.Helper
import portal.Prihlasovanie

import org.openqa.selenium.Keys as Keys

def filePath = RunConfiguration.getProjectDir()

def priloha = filePath + '/Data Files/Dokument (1).pdf'

Mail mail = new Mail()

Helper help = new Helper()

Prihlasovanie prihlasovanie = new Prihlasovanie()

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021625')

WebUI.waitForJQueryLoad(20)
'Zoradenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Prihlky a rozhodnutia  ePrihlky/button_(nepovinn)_btn-zoradit-podla-predvolene'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Prihlky a rozhodnutia  ePrihlky/label_Poda priezviska (abecedne - vzostupne_7a4950'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Prihlky a rozhodnutia  ePrihlky/button_Sp_btn-zoradit govuk-button govuk-bu_4b9abc'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Prihlky a rozhodnutia  ePrihlky/button_Detail_govuk-button govuk-button--se_40abef'))

String priezvisko = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/div_Priezvisko_dietaPriezvisko'))

String meno = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/div_Meno_dietaMeno'))

String datumNarodenia = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/DatumNrodeniaZiaka'))

String identifikator = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/identifikatorPrihlaskyZS'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_stavPrihlasky badge green'), 
    'Podaná')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_skola-status-badge green'), 
    'Podaná')

'Vyžiadanie prílohy'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/button_Neboli nahran iadne prlohy_btn-vyzia_b059b7'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Vyiada prlohu  ePrihlky/select_(nepovinn)_select-TypPrilohySelect'), 
    '3', true)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Vyiada prlohu  ePrihlky/textarea_(nepovinn)_textarea-dovodText'), 
    'Príloha k ZŠ')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Vyiada prlohu  ePrihlky/button_Sp_btn-odoslat govuk-button govuk-bu_528af4'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_stavPrihlasky badge red'), 
    'Neúplná')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_skola-status-badge red'), 
    'Neúplná')

'Odvolať žiadosť'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/button_Dvod_data-prihlaska-odvolat-ziadost _a29034'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/button_Nie_btn-confirm govuk-button govuk-b_cf1cc7'))

WebUI.delay(4)
//Kontrola odvolania
teloMailu = mail.getLastEmailText('pop.gmail.com', 'pop3', GlobalVariable.mailLogin, GlobalVariable.mailHeslo)
teloMailu = help.cleanupCidUrls(teloMailu)
teloMailu = teloMailu.replaceAll(/\r?\n+/, ' ').replaceAll(/\s+/, ' ').trim()
assert teloMailu.equals('Vážený/á pán/pani Tomáš Lukáč,  radi by sme vás informovali, že požiadavka na doloženie dodatočných dokumentov príloh k Vašej prihláške zaevidovanej v portáli Elektronické prihlášky do škôl bola zrušená. Nie je teda potrebné dodatočne nahrávať  žiadne ďalšie prílohy k prihláške pre: '+meno+' '+priezvisko+' nar. '+datumNarodenia+' .  Ak ste už zadali dokumenty na základe  predchádzajúceho odkazu, upozorňujeme, že tento odkaz je už neaktívny.  V prípade akýchkoľvek otázok nás neváhajte kontaktovať.  S pozdravom Tím elektronických prihlášok MŠVVaM SR Tento email bol generovaný automaticky portálom Elektronické prihlášky do škôl, ktorý je v správe Ministerstva školstva, výskumu, vývoja a mládeže Slovenskej republiky. Neodpovedajte naň."')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/button_Neboli nahran iadne prlohy_btn-vyzia_b059b7'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Vyiada prlohu  ePrihlky/select_(nepovinn)_select-TypPrilohySelect'), 
    '3', true)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Vyiada prlohu  ePrihlky/textarea_(nepovinn)_textarea-dovodText'), 
    'Príloha k ZŠ')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Vyiada prlohu  ePrihlky/button_Sp_btn-odoslat govuk-button govuk-bu_528af4'))

WebUI.delay(1)
//Kontrola vyžiadania
teloMailu = mail.getLastEmailText('pop.gmail.com', 'pop3', GlobalVariable.mailLogin, GlobalVariable.mailHeslo)
teloMailu = help.cleanupCidUrls(teloMailu)
teloMailu = teloMailu.replaceAll(/\r?\n+/, ' ').replaceAll(/\s+/, ' ').trim()
assert teloMailu.equals('Vážený/á pán/pani Tomáš Lukáč, pri kontrole prihlášky '+identifikator+' pre školu Základná škola pre AT pre '+meno+' '+priezvisko+' sme zistili, že je potrebné doložiť nasledujúcu prílohu: Čestné vyhlásenie zákonného zástupcu z dôvodu že " Príloha k ZŠ ". Prosíme Vás o doplnenie požadovanej prílohy k prihláške. Pre podrobnejšie informácie o stave Vašej prihlášky a priebehu jej spracovania sa sa prihláste na portáli Elektronické prihlášky do škôl.   Link na prihlásenie S pozdravom Tím elektronických prihlášok MŠVVaM SR Tento email bol generovaný automaticky portálom Elektronické prihlášky do škôl, ktorý je v správe Ministerstva školstva, výskumu, vývoja a mládeže Slovenskej republiky. Neodpovedajte naň."')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_skola-status-badge red'), 
    'Neúplná')

//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/span_R_sprava-nazov'), 
 //   'Riaditeľ školy Základná škola pre AT požadoval ďalšie prílohy.')

WebUI.verifyTextPresent('Riaditeľ školy Základná škola pre AT požadoval ďalšie prílohy.', false)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/b'), 
    'Žiadosť o doplnenie prílohy bola úspešne odoslaná')

prihlasovanie.odhlasPouzivatela()

prihlasovanie.prihlasPouzivatela('ljxikynq7v@dollicons.com', 'w1oXMoeykcdLiib/wAKM5A==', false, GlobalVariable.F2A)

//nájdenie správnej prihlášky podľa mena dieťaťa
int cislo = 5

while (cislo <= 1000) {
	String xpath = ('//div[' + cislo) + ']/div/div/div/div[2]/span'

	TestObject dynamicObj = new TestObject()

	dynamicObj.addProperty('xpath', ConditionType.EQUALS, xpath)

	if (WebUI.getText(dynamicObj) == meno + " " + priezvisko) {
		break
	}
	
	cislo++
}

WebUI.verifyTextPresent('Riaditeľ základnej školy požaduje doplnenie príloh. Pridanie prílohy nájdete v stĺpci Akcia.', false)

//stav prihlášky
TestObject stavPrihlaskyBadge = new TestObject()

stavPrihlaskyBadge.addProperty('xpath', ConditionType.EQUALS, ('//div[' + cislo.toString()) + ']/div[1]/div[3]/div[1]/table[2]/tbody[1]/tr[1]/td[4]/span[1]/span[1]')

WebUI.verifyElementText(stavPrihlaskyBadge, 'Neúplná')
/**
//kontrola banneru
TestObject bannerPrilohy = new TestObject()

bannerPrilohy.addProperty('xpath', ConditionType.EQUALS, ('//div[' + cislo.toString()) + ']/div/div[2]/div/div/div[2]/span/b')

WebUI.verifyElementText(bannerPrilohy, 'Nahrajte prílohy', FailureHandling.OPTIONAL)
**/
//pridanie prílohy do prihlášky
TestObject pridajPrilohuButton = new TestObject()

pridajPrilohuButton.addProperty('xpath', ConditionType.EQUALS, ('//div[' + cislo.toString()) + ']/div/div[3]/div/table[2]/tbody/tr/td[5]/span/a')

WebUI.click(pridajPrilohuButton)


//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Moje prihlky  ePrihlky/span_Riadite koly poaduje alie prlohy_prihl_48cb18'), 'Neúplná')

//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Moje prihlky  ePrihlky/bannerPrilohy'), 'Nahrajte prílohy')

//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Moje prihlky  ePrihlky/span_warning_panel-text'), 'Nahrajte prílohy\nRiaditeľ základnej školy požaduje doplnenie príloh. Pridanie prílohy nájdete v stĺpci Akcia.')

//'Pridať prílohu'
//WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Moje prihlky  ePrihlky/a_Nepln_data-prihlaska-pridat-prilohu govuk_a64c43'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Priloi dokumenty  ePrihlky/div_Nahran_govuk-accordion__section-desc-te_b9349f'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Priloi dokumenty  ePrihlky/div_Dvod_sprava-text'), 
    'Príloha k ZŠ')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Priloi dokumenty  ePrihlky/div_R_sprava-od'), 
    'Riaditeľ školy Základná škola pre AT požadoval ďalšie prílohy.')

WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Priloi dokumenty  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b'),priloha)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Priloi dokumenty  ePrihlky/button_Sp_btn-odoslat govuk-button govuk-bu_528af4'))

//Kontrola doplnenia
teloMailu = mail.getLastEmailText('pop.gmail.com', 'pop3', GlobalVariable.mailLogin, GlobalVariable.mailHeslo)
teloMailu = help.cleanupCidUrls(teloMailu)
teloMailu = teloMailu.replaceAll(/\r?\n+/, ' ').replaceAll(/\s+/, ' ').trim()
assert teloMailu.equals('Vážený/á pán/pani/ Tomáš Lukáč,  dovoľujeme si Vás informovať, že k Vašej prihláške do Základná škola pre AT pre '+meno+' '+priezvisko+',  zaevidovanej v elektronickom portáli prihlášok bola doručená príloha s názvom Čestné vyhlásenie zákonného zástupcu. Doručenú prílohu si prosím starostlivo skontrolujte prihlásením sa na portáli Elektronických prihlášok v detaile prihlášky, alebo v prílohe tohto mailu. Prihlásením sa na portáli zároveň získate aj ďalšie informácie o stave Vašej prihlášky a priebehu jej spracovania.   Link na prihlásenie S pozdravom Tím elektronických prihlášok MŠVVaM SR Tento email bol generovaný automaticky portálom Elektronické prihlášky do škôl, ktorý je v správe Ministerstva školstva, výskumu, vývoja a mládeže Slovenskej republiky. Neodpovedajte naň."')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Priloi dokumenty  ePrihlky/a_Vaa prihlka bude oskoro posden. akujeme z_527046'))

cislo = 5

while (cislo <= 1000) {
	String xpath = ('//div[' + cislo) + ']/div/div/div/div[2]/span'

	TestObject dynamicObj = new TestObject()

	dynamicObj.addProperty('xpath', ConditionType.EQUALS, xpath)

	if (WebUI.getText(dynamicObj) == meno + " " + priezvisko) {
		break
	}
	
	cislo++
}

//stav prihlášky
TestObject stavPrihlaskyBadgeUpdate = new TestObject()

stavPrihlaskyBadgeUpdate.addProperty('xpath', ConditionType.EQUALS, ('//div[' + cislo.toString()) + ']/div[1]/div[3]/div[1]/table[2]/tbody[1]/tr[1]/td[4]/span[1]/span[1]')

WebUI.verifyElementText(stavPrihlaskyBadgeUpdate, 'Doplnená')

//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Moje prihlky  ePrihlky/span_Riadite koly poaduje alie prlohy_prihl_fa7705'), 'Doplnená')

prihlasovanie.odhlasPouzivatela()

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021625')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Prihlky a rozhodnutia  ePrihlky/input_Povinn prihlky na prijatie, ale kvli _9491e4'), 
    meno + " " + priezvisko)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Prihlky a rozhodnutia  ePrihlky/button_Povinn prihlky na prijatie, ale kvli_21db34'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Prihlky a rozhodnutia  ePrihlky/button_Detail_govuk-button govuk-button--se_40abef'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_stavPrihlasky badge green_1'), 
    'Doplnená')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Prilohy/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_skola-status-badge green_1'), 
    'Doplnená')

