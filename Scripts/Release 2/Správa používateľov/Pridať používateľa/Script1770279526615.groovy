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

prihlasovanie.prihlasRiaditela('930570706', 'uEdivOPFtSGvP7ePRyzmOg==', GlobalVariable.F2A, '910020859')

'Správa školy'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/span'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/a_Profil koly_govuk-header__link'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/h1_Uloi zmeny_govuk-heading-xl'), 
    'Správa používateľov')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/p_Sprva pouvateov_subtitle-riad-sprava-pouz_02e8ee'), 
    'Spravujte používateľov a priraďte im rolu.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/h3_Sprva pouvateov_govuk-heading-m'), 
    'Používatelia')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/span_1'), 
    'Martin Martinský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/div'), 
    'EDUID: 930570706')

'Pridat používateľa - spracovateľ'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/span_2'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/div_Prida pouvatea_idsk-subtitle'), 
    'Pridať používateľa')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/div_Prida pouvatea_idsk-card__description'), 
    'Vyhľadajte a zapojte odborníkov z vašej školy do procesu kontroly prihlášok.')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/div_Povinn polia s oznaen_sprava-pouzivatel_b8fd3d'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/button_close_govuk-button'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/span_3'), 
    'Klaudia Kelmová')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/div_1'), 
    'EDUID: 930593021')

'Pridať používateľa - administrátor'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/span_2'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/div_Povinn polia s oznaen_sprava-pouzivatel_b8fd3d_1'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/select_(nepovinn)_select-novaRoleSelect'), 
    '2', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/button_close_govuk-button'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/span_4'), 
    'Dušan Lemo')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Sprva koly  ePrihlky/div_2'), 
    'EDUID: 930593019')

prihlasovanie.odhlasPouzivatela()

prihlasovanie.prihlasRiaditela('930593021', 'ZAUGwfApXs7u0wA8TalYuA==', GlobalVariable.F2A, '910020859')

'Profil spracovateľa'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Prihlky a rozhodnutia  ePrihlky/span_Menu_inicialy-osoby-header'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Prihlky a rozhodnutia  ePrihlky/a'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Mj profil  ePrihlky/div_kola_profil-riaditel-skola'), 
    'Gymnázium Metodova', FailureHandling.OPTIONAL)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Mj profil  ePrihlky/div_Pozcia_profil-riaditel-typ'), 
    'Spracovateľ')

prihlasovanie.odhlasPouzivatela()

prihlasovanie.prihlasRiaditela('930593019', 'lNz7Bw/ivI+44MMBW0nO0A==', GlobalVariable.F2A, '910020859')

'Profil administrátora'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Prihlky a rozhodnutia  ePrihlky/span_Menu_inicialy-osoby-header_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Prihlky a rozhodnutia  ePrihlky/a'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Mj profil  ePrihlky/div_kola_profil-riaditel-skola'), 
    'Gymnázium Metodova', FailureHandling.OPTIONAL)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/PridatPouzivatela/Page_Mj profil  ePrihlky/div_Pozcia_profil-riaditel-typ_1'), 
    'Administrátor')

prihlasovanie.odhlasPouzivatela()

