package portal;
import internal.GlobalVariable;
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint;
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase;
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData;
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject;
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject;

import Mail
import com.kms.katalon.core.annotation.Keyword;
import com.kms.katalon.core.checkpoint.Checkpoint;
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords;
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords;
import com.kms.katalon.core.model.FailureHandling;
import com.kms.katalon.core.testcase.TestCase;
import com.kms.katalon.core.testdata.TestData;
import com.kms.katalon.core.testobject.TestObject;
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords;
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI;
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords;

public class Prihlasovanie {
	Mail email = new Mail()

	/**
	 * Prihlási sa na portál ako ZZ.
	 *
	 * @param meno                  Prihlasovacie meno (e-mail).
	 * @param heslo            		Heslo k účtu.
	 * @param dvojFaktorovka        2-faktorová autentifikácia. Berie sa z globálnej premennej.
	 * @param upvs					Príznak či ide o UPVS. Default je false.
	 **/
	def prihlasPouzivatela(String meno, String heslo, boolean upvs = false, boolean dvojFaktorovka) {
		WebUI.click(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Pre koly_govuk-button govuk-button__basic'))

		WebUI.click(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_- Iam.Web/h3_account_circle_govuk-heading-m'))

		WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_Prihlasovacie meno a heslo - Iam.Web/input_Prihlasovacie meno je V e-mail vo for_3e8d95'),
				meno)

		WebUI.setEncryptedText(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_Prihlasovacie meno a heslo - Iam.Web/input__UserPassword'),
				heslo)

		WebUI.click(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_Prihlasovacie meno a heslo - Iam.Web/input_Zabudli ste heslo_btn-continue'))

		if (dvojFaktorovka) {
			def kod = email.getSixDigitNumberFromLastEmail('pop.gmail.com', 'pop3', 'katalontest987@gmail.com', GlobalVariable.mailHeslo)
			WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Overovac kd - Iam.Web/input__OneTimeCode'), kod.toString())
			WebUI.click(findTestObject('Object Repository/Zak_test/Page_Overovac kd - Iam.Web/input__btn-continue'))
		}

		//Prihlasovanie upvs
		if(upvs) {
			WebUI.click(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Pre koly_govuk-button govuk-button__basic'))

			WebUI.click(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_- Iam.Web/h3_contact_emergency_govuk-heading-m'))

			WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_slovensko.sk - Prihlsenie/input_Prihlasovacie meno_username'),
					'E0005252428')

			WebUI.setEncryptedText(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_slovensko.sk - Prihlsenie/input_Heslo_password'),
					'2C85ycFzMIs9RpPQf+Ep/w==')

			WebUI.click(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_slovensko.sk - Prihlsenie/button_Heslo_button button--wider'))
		}
	}

	/**
	 * Prihlásenie sa na portál ako Osoba konajúca za SaSZ.
	 *
	 * @param eduid                 Prihlasovacie meno (EDUID).
	 * @param heslo            		Heslo k účtu.
	 * @param dvojFaktorovka        2-faktorová autentifikácia. Berie sa z globálnej premennej.
	 * @param eduidSkoly			Slúži na vybranie hodnoty zo selectu, ak má riaditeľ viacero škôl.
	 **/
	def prihlasRiaditela(String eduid, String heslo, boolean dvojFaktorovka, String eduidSkoly) {
		WebUI.click(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Menu_govuk-button govuk-button--sec'))

		WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_Vstup koly do portlu - Iam.Web/input_Zadajte prihlasovacie meno vo formte _c099e3'),eduid)

		WebUI.setEncryptedText(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_Vstup koly do portlu - Iam.Web/input__Password'),heslo)

		WebUI.click(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_Vstup koly do portlu - Iam.Web/input_Zabudli ste heslo_btn-continue'))

		if (dvojFaktorovka) {
			def kod = email.getSixDigitNumberFromLastEmail('pop.gmail.com', 'pop3', 'katalontest987@gmail.com', GlobalVariable.mailHeslo)
			WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Overovac kd - Iam.Web/input__OneTimeCode'), kod.toString())
			WebUI.click(findTestObject('Object Repository/Zak_test/Page_Overovac kd - Iam.Web/input__btn-continue'))
		}

		if (WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_ePrihlky/select_(nepovinn)_select-skola'),5, FailureHandling.OPTIONAL)) {

			//WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_ePrihlky/select_(nepovinn)_select-skola'),'910019568', true)

			WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_ePrihlky/select_(nepovinn)_select-skola'),
					eduidSkoly, true)

			WebUI.click(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_ePrihlky/button_keyboard_arrow_down_govuk-button gov_131c98'))
		}
	}

	/**
	 * Odhlásenie používateľa z privátnej zóny.
	 * 
	 */
	def odhlasPouzivatela() {
		WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/div_Menu_profile-photo'))

		WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/a_Oznmenia_logoutBtn'))
	}
}